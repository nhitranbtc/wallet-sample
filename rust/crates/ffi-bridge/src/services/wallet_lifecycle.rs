//! Wallet-lifecycle helpers: `create_wallet` and
//! `restore_wallet_via_native_surface`.
//!
//! These helpers are crate-internal (`pub(super)`) and own the only
//! places in `ffi-bridge` that import [`keystore::Mnemonic`] and
//! [`keystore::WalletSession`] — `crate::api` stays free of those
//! identifiers so the falsifier
//! `no_zeroize_type_in_ffi_api_surface` (Task 14) stays green.

use crate::error::DartError;
use crate::handle::WalletHandle;
use crate::summary::WalletSummary;

use keystore::{Mnemonic, WalletSession};
use secure_storage::{BiometricProof, Vault};
use wallet_domain::account::ChainId;
use wallet_domain::descriptor::ChainDescriptor;
use wallet_domain::error::ErrorCategory;

/// Build a wallet: generate a fresh 24-word mnemonic, hand it to the
/// native secure-storage surface (the platform-call body is the
/// single `unimplemented!()` site allowed inside these helpers),
/// persist an AES-256-GCM `Vault`, activate the session, and derive
/// account-zero addresses for every chain in the registry.
pub(crate) fn create(handle: &WalletHandle) -> Result<WalletSummary, DartError> {
    let mnemonic =
        Mnemonic::generate(24).map_err(|e| DartError::from(&e, ErrorCategory::Vault))?;

    // Platform secure-storage call site: real implementations issue
    // an OS-level biometric prompt here and wrap the DEK with the
    // platform keystore. The body is intentionally `unimplemented!()`
    // — Task 11 ships the surface contract, the platform shim is
    // Task 12.
    let _wrapped_key: secure_storage::WrappedKey = unimplemented!(
        "native secure-storage wrap_key for create_wallet — Task 12"
    );

    // Persist the encrypted vault. The proof-stage DEK is a
    // placeholder; the real DEK is the platform-wrapped key fetched
    // above. `mnemonic.seed()` returns `&[u8; 64]`; we slice to
    // `&[u8]` to satisfy `Vault::encrypt`'s plaintext parameter.
    let dek: [u8; 32] = [0u8; 32];
    let nonce: [u8; 12] = std::array::from_fn(|i| (i as u8).wrapping_mul(31));
    let plaintext: &[u8] = mnemonic.seed();
    let _blob = Vault::encrypt(&dek, &nonce, plaintext)
        .map_err(|e| DartError::from(&e, ErrorCategory::Vault))?;

    // Activate the session: Uninitialized -> Enrolling -> Ready.
    let session_mnemonic =
        Mnemonic::generate(24).map_err(|e| DartError::from(&e, ErrorCategory::Vault))?;
    let session = WalletSession::from_mnemonic(session_mnemonic)
        .map_err(|e| DartError::from(&e, ErrorCategory::Vault))?;

    let session_arc = handle.inner().clone();
    {
        let mut guard = session_arc.lock().expect("session mutex");
        guard
            .begin_enroll(mnemonic)
            .map_err(|e| DartError::from(&e, ErrorCategory::Vault))?;
        guard
            .activate(session)
            .map_err(|e| DartError::from(&e, ErrorCategory::Vault))?;
    }

    let accounts = account_zero_addresses_for_session();
    let wallet_id = uuid::Uuid::new_v4().to_string();
    Ok(WalletSummary { wallet_id, accounts })
}

/// Restore a wallet via the native platform surface. The platform
/// restore prompt body is intentionally `unimplemented!()` — the
/// contract is the surface, the impl is Task 12.
pub(crate) fn restore_via_native_surface(
    handle: &WalletHandle,
) -> Result<WalletSummary, DartError> {
    // Platform secure-storage call site.
    let _proof: BiometricProof = unimplemented!(
        "native secure-storage restore-wallet biometric + vault rewrap"
    );
    // Touch the handle so the unused warning is silenced if this stub
    // is replaced by a one-shot call instead.
    let _ = handle.inner();
    let accounts = account_zero_addresses_for_session();
    let wallet_id = uuid::Uuid::new_v4().to_string();
    Ok(WalletSummary { wallet_id, accounts })
}

// ---------------------------------------------------------------------------
// helpers (private to this module)
// ---------------------------------------------------------------------------

/// Account-zero `(chain, descriptor, address)` tuples for
/// [`WalletSummary`]. Real derivation is via `keystore::Derive`; we
/// currently return the placeholder trio until Task 14 wires the
/// live session into the FFI.
fn account_zero_addresses_for_session() -> Vec<(ChainId, ChainDescriptor, String)> {
    account_zero_descriptors()
        .into_iter()
        .map(|d| {
            let chain = d.chain;
            (chain, d, placeholder_address(chain))
        })
        .collect()
}

/// Account-zero descriptors for every Release-1 chain.
fn account_zero_descriptors() -> Vec<ChainDescriptor> {
    vec![
        ChainDescriptor {
            chain: ChainId::Ethereum,
            symbol: "ETH".into(),
            default_decimals: 18,
        },
        ChainDescriptor {
            chain: ChainId::Bitcoin,
            symbol: "BTC".into(),
            default_decimals: 8,
        },
    ]
}

/// Testnet-shaped placeholder receive addresses for the stub phase.
fn placeholder_address(chain: ChainId) -> String {
    match chain {
        ChainId::Ethereum => "0x1111111111111111111111111111111111111111".into(),
        ChainId::Bitcoin => "tb1qexamplewalletaddress0000000000000000000".into(),
        ChainId::Solana => "11111111111111111111111111111111111111111111".into(),
        ChainId::Tron => "TExampleTronAddress1111111111111111111111".into(),
    }
}
