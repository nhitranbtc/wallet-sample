//! Opaque handle types threaded through the eleven-method FFI surface.
//!
//! `WalletHandle` carries the active [`SessionState`] (a single
//! per-process session). `PreparedHandle` carries a single
//! [`PreparedTransfer`] behind a mutex so the bridge can hand the
//! opaque ID back to Dart for the sign/broadcast call without
//! exposing the transfer directly.
//!
//! Both impl the crate-private `sealed::Sealed` trait; nothing outside
//! `lib.rs` of this crate can pattern-match on these types.

use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use wallet_domain::transfer::PreparedTransfer;
use wallet_orchestration::SessionState;

/// Per-wallet FFI handle. Cloning the `Arc` re-uses the same session
/// across calls; locking the inner mutex serializes all reads/writes
/// against [`SessionState`].
pub struct WalletHandle {
    inner: Arc<Mutex<SessionState>>,
}

impl WalletHandle {
    /// Build a fresh handle backed by a brand-new [`SessionState`].
    /// Kept `pub(crate)` — the FFI surface receives handles from
    /// `bridge_facade.dart`, never constructed on the Rust side.
    pub(crate) fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(SessionState::new())),
        }
    }

    /// Build a handle backed by an explicit `SessionState`. Used by
    /// tests and by callers that have already constructed the session.
    pub(crate) fn with_session(session: SessionState) -> Self {
        Self {
            inner: Arc::new(Mutex::new(session)),
        }
    }

    /// Borrow the inner `Arc<Mutex<SessionState>>`. The FFI surface
    /// locks this to read phase / mutate session state.
    pub(crate) fn inner(&self) -> &Arc<Mutex<SessionState>> {
        &self.inner
    }
}

/// Opaque token that points at a single in-flight [`PreparedTransfer`].
/// The struct holds the transfer behind `Arc<Mutex<Option<...>>>` so
/// [`authenticate_sign_and_broadcast`] can take ownership of it without
/// the FFI boundary ever observing the chain-specific payload.
pub struct PreparedHandle {
    id: String,
    expires_at: DateTime<Utc>,
    payload: Arc<Mutex<Option<PreparedTransfer>>>,
}

impl PreparedHandle {
    /// Construct an empty handle (`None` payload). Used when only the
    /// handle metadata (id, expiry) is needed.
    pub fn new(id: String, expires_at: DateTime<Utc>) -> Self {
        Self {
            id,
            expires_at,
            payload: Arc::new(Mutex::new(None)),
        }
    }

    /// Construct a handle that already carries a `PreparedTransfer`.
    /// Used by `prepare_native_transfer` so the sign/broadcast step
    /// can take the transfer back without round-tripping through Dart.
    pub(crate) fn with_payload(
        id: String,
        expires_at: DateTime<Utc>,
        prepared: PreparedTransfer,
    ) -> Self {
        Self {
            id,
            expires_at,
            payload: Arc::new(Mutex::new(Some(prepared))),
        }
    }

    /// Opaque handle identifier. The Dart-side facade uses this as the
    /// user-visible token; the Rust side looks up via [`Self::take_payload`].
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Time at which the preparation expires. Sign/broadcast calls past
    /// this moment must reject with `fresh_preparation_required = true`.
    pub fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }

    /// Take ownership of the wrapped `PreparedTransfer`. Returns `None`
    /// if a previous call already consumed it, or if the handle was
    /// constructed via [`Self::new`] without a payload.
    pub(crate) fn take_payload(&self) -> Option<PreparedTransfer> {
        self.payload.lock().expect("prepared handle mutex").take()
    }
}
