use keystore::{Mnemonic, WalletSession};
use std::sync::atomic::{AtomicU8, Ordering};
use wallet_domain::error::WalletError;
use zeroize::ZeroizeOnDrop;

/// The five phases a wallet session can be in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Phase {
    Uninitialized = 0,
    Enrolling = 1,
    Ready = 2,
    Locked = 3,
    Removed = 4,
}

#[derive(ZeroizeOnDrop)]
pub struct SessionState {
    /// Stored in an `AtomicU8` so consumers can perform non-blocking
    /// phase checks on `Ordering::Acquire` if they want a relaxed read.
    /// The phase tag itself is not secret — `AtomicU8` does not impl
    /// `Zeroize` — so we mark it `#[zeroize(skip)]`.
    #[zeroize(skip)]
    phase: AtomicU8,
    mnemonic: Option<Mnemonic>,
    session: Option<WalletSession>,
}

impl SessionState {
    pub fn new() -> Self {
        Self {
            phase: AtomicU8::new(Phase::Uninitialized as u8),
            mnemonic: None,
            session: None,
        }
    }

    pub fn begin_enroll(&mut self, mnemonic: Mnemonic) -> Result<(), WalletError> {
        if self.phase() != Phase::Uninitialized {
            return Err(WalletError::Locked);
        }
        self.mnemonic = Some(mnemonic);
        self.phase = AtomicU8::new(Phase::Enrolling as u8);
        Ok(())
    }

    /// Move from `Enrolling` to `Ready`. The mnemonic is `take()`-d
    /// from the option (which `drop`s it; `Mnemonic` derives
    /// `ZeroizeOnDrop`), then `zeroize_phrase()` is invoked explicitly
    /// to guarantee the phrase bytes are scrubbed regardless of drop
    /// ordering.
    pub fn activate(&mut self, session: WalletSession) -> Result<(), WalletError> {
        if self.phase() != Phase::Enrolling {
            return Err(WalletError::Locked);
        }
        if let Some(mut m) = self.mnemonic.take() {
            m.zeroize_phrase();
        }
        self.session = Some(session);
        self.phase = AtomicU8::new(Phase::Ready as u8);
        Ok(())
    }

    pub fn lock(&mut self) {
        // Terminal state: a removed wallet has no session and no
        // mnemonic to scrub. `lock()` after `remove()` must be a no-op
        // so the phase tag keeps its terminal meaning — re-marking it
        // `Locked` would let a downstream check conflate the two and
        // resurrect a destroyed session from a stray UI flow.
        if matches!(self.phase(), Phase::Removed) {
            return;
        }
        if self.session.take().is_some() {
            // Dropped with `ZeroizeOnDrop`.
        }
        self.phase = AtomicU8::new(Phase::Locked as u8);
    }

    pub fn remove(&mut self) {
        let _ = self.session.take();
        if let Some(mut m) = self.mnemonic.take() {
            m.zeroize_phrase();
        }
        self.phase = AtomicU8::new(Phase::Removed as u8);
    }

    fn phase(&self) -> Phase {
        match self.phase.load(Ordering::SeqCst) {
            0 => Phase::Uninitialized,
            1 => Phase::Enrolling,
            2 => Phase::Ready,
            3 => Phase::Locked,
            _ => Phase::Removed,
        }
    }

    /// Returns `true` if the current phase (under `Acquire` ordering) is
    /// `Ready`. Public for `signing_coordinator` consumption.
    pub fn is_ready(&self) -> bool {
        self.phase.load(Ordering::Acquire) == Phase::Ready as u8
    }

    /// Returns `true` if the current phase is `Removed`.
    pub fn is_removed(&self) -> bool {
        self.phase.load(Ordering::Acquire) == Phase::Removed as u8
    }

    /// Test-only constructor: returns a `SessionState` already in
    /// `Ready` with a real `WalletSession` derived from a real
    /// `Mnemonic`. Gated behind `#[cfg(test)]` so it is never compiled
    /// into release artifacts; the `_for_test` suffix and
    /// `#[doc(hidden)]` mark it as not part of the production API.
    ///
    /// Construction routes through the public state machine —
    /// `begin_enroll` then `activate` — so any future tightening of
    /// phase guards or `WalletSession` invariants is exercised by
    /// tests instead of being skipped over by a side door.
    #[cfg(test)]
    #[doc(hidden)]
    pub fn ready_for_test() -> Self {
        use keystore::{Mnemonic, WalletSession};

        // Trezor's well-known BIP39 test vector: zero entropy, but
        // valid phrase and deterministic 64-byte seed.
        const TREZOR_PHRASE: &str = "abandon abandon abandon abandon \
            abandon abandon abandon abandon abandon abandon abandon about";

        // `Mnemonic` derives `ZeroizeOnDrop` and is not `Clone`, so
        // derive two copies from the same phrase. BIP39 makes this
        // deterministic — both `seed`s are byte-identical.
        let mnemonic_for_enroll = Mnemonic::from_phrase(TREZOR_PHRASE, "")
            .expect("Trezor test mnemonic parses");
        let mnemonic_for_session = Mnemonic::from_phrase(TREZOR_PHRASE, "")
            .expect("Trezor test mnemonic parses");
        let wallet_session =
            WalletSession::from_mnemonic(mnemonic_for_session).expect("session from mnemonic");

        let mut s = Self::new();
        s.begin_enroll(mnemonic_for_enroll).expect("begin_enroll");
        s.activate(wallet_session).expect("activate");
        s
    }
}

impl Default for SessionState {
    fn default() -> Self {
        Self::new()
    }
}