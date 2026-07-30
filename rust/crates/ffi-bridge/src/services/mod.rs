//! Internal service helpers for the eleven-method FFI surface.
//!
//! `src/api.rs` is the **frozen** surface — it must contain only the
//! eleven `pub fn` declarations and their one-line forwarders. Real
//! implementations (and any helper that needs to import `Mnemonic`,
//! `WalletSession`, etc. — types that the falsifier
//! `no_zeroize_type_in_ffi_api_surface` bans from the FFI surface
//! itself) live in these private modules. Each submodule is
//! `pub(super)` so the helper symbols cannot leak past `crate::api`.

pub(super) mod destructive;
pub(super) mod signing;
pub(super) mod wallet_lifecycle;
pub(super) mod wallet_status;
