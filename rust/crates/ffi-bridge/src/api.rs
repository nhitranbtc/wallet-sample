//! The **frozen** Dart-facing surface.
//!
//! This file MUST contain exactly eleven `pub fn` definitions and no
//! others. The companion snapshot test in
//! `tests/surface_snapshot_test.rs` enforces that contract on every
//! change.
//!
//! Every function may return a [`crate::error::DartError`] so leaked
//! secrets (addresses, hashes, base58 payloads) cannot reach Dart.
//! Platform secure-storage calls are the **only** sites where
//! `unimplemented!()` is permitted; those bodies have been moved into
//! the `crate::services::*` helper modules so the sensitive
//! keystore-side identity types never appear at module scope here
//! (Task 14 falsifier `no_zeroize_type_in_ffi_api_surface`).
//!
//! Real implementations live in:
//! - `services::wallet_lifecycle` — `create_wallet`,
//!   `restore_wallet_via_native_surface`
//! - `services::signing` — `authenticate_sign_and_broadcast`
//! - `services::destructive` — `lock_wallet`, `remove_wallet`
//! - `services::wallet_status` — `wallet_status`, `list_chains`,
//!   `refresh_accounts`, `prepare_native_transfer`,
//!   `watch_transfer_status`, `get_receive_address`

use crate::error::DartError;
use crate::handle::{PreparedHandle, WalletHandle};
use crate::services::{destructive, signing, wallet_lifecycle, wallet_status};
use crate::status::WalletStatus;
use crate::summary::WalletSummary;

use wallet_domain::account::ChainId;
use wallet_domain::broadcast::TransactionStatus;
use wallet_domain::descriptor::ChainDescriptor;

/// Build a wallet: generate a fresh 24-word mnemonic, hand it to the
/// native secure-storage surface, persist an AES-256-GCM `Vault`,
/// activate the session, and derive account-zero addresses for every
/// chain in the registry. Implementation: [`wallet_lifecycle::create`].
pub fn create_wallet(handle: &WalletHandle) -> Result<WalletSummary, DartError> {
    wallet_lifecycle::create(handle)
}

/// Restore a wallet via the native platform surface. Implementation:
/// [`wallet_lifecycle::restore_via_native_surface`].
pub fn restore_wallet_via_native_surface(
    handle: &WalletHandle,
) -> Result<WalletSummary, DartError> {
    wallet_lifecycle::restore_via_native_surface(handle)
}

/// Read-only view onto the session. Implementation:
/// [`wallet_status::read`].
pub fn wallet_status(handle: &WalletHandle) -> WalletStatus {
    wallet_status::read(handle)
}

/// Release-1 chain set: Ethereum (Sepolia) + Bitcoin (testnet).
/// Implementation: [`wallet_status::list_chains`].
pub fn list_chains(handle: &WalletHandle) -> Vec<ChainId> {
    wallet_status::list_chains(handle)
}

/// Refresh account-zero descriptors for the wallet. Implementation:
/// [`wallet_status::refresh_accounts`].
pub fn refresh_accounts(handle: &WalletHandle) -> Vec<ChainDescriptor> {
    wallet_status::refresh_accounts(handle)
}

/// Build a [`crate::handle::PreparedHandle`] wrapping a prepared
/// transfer for the chosen chain. Implementation:
/// [`wallet_status::prepare_native_transfer`].
pub fn prepare_native_transfer(
    handle: &WalletHandle,
    chain: ChainId,
    recipient: String,
    amount: u128,
) -> Result<PreparedHandle, DartError> {
    wallet_status::prepare_native_transfer(handle, chain, recipient, amount)
}

/// Consume a platform-issued [`secure_storage::BiometricProof`] and
/// sign + broadcast the prepared transfer. Implementation:
/// [`signing::authenticate_sign_and_broadcast`].
pub fn authenticate_sign_and_broadcast(
    handle: &WalletHandle,
    prepared: &PreparedHandle,
) -> Result<String, DartError> {
    signing::authenticate_sign_and_broadcast(handle, prepared)
}

/// Look up the current status of a transaction by id. Implementation:
/// [`wallet_status::watch_transfer_status`].
pub fn watch_transfer_status(transaction_id: String) -> Result<TransactionStatus, DartError> {
    wallet_status::watch_transfer_status(transaction_id)
}

/// Return a receive address for the given chain. Implementation:
/// [`wallet_status::get_receive_address`].
pub fn get_receive_address(
    handle: &WalletHandle,
    chain: ChainId,
) -> Result<String, DartError> {
    wallet_status::get_receive_address(handle, chain)
}

/// Lock the active wallet. Implementation: [`destructive::lock`].
pub fn lock_wallet(handle: &WalletHandle) -> Result<(), DartError> {
    destructive::lock(handle)
}

/// Remove the wallet entirely. Implementation: [`destructive::remove`].
pub fn remove_wallet(handle: &WalletHandle) -> Result<(), DartError> {
    destructive::remove(handle)
}
