//! `ffi-bridge` — the **frozen** Dart-facing surface.
//!
//! This crate exposes exactly eleven `pub fn` entrypoints through
//! [`api`]; the count is enforced by the snapshot test in
//! `tests/surface_snapshot_test.rs`. The handle types
//! ([`handle::WalletHandle`], [`handle::PreparedHandle`]) are
//! crate-sealed: a private `sealed::Sealed` trait is impl'd for each
//! of them inside this module so that no foreign code can pattern-match
//! on their public fields. The only path to construct or inspect a
//! handle is via the Flutter `bridge_facade.dart` generated bindings.
//!
//! `flutter_rust_bridge` consumes this crate via
//! `flutter_rust_bridge.yaml`; codegen emits the Dart bindings the
//! Flutter app uses.

pub mod api;
pub mod error;
pub mod handle;
pub mod status;
pub mod summary;

pub mod sealed {
    /// Private trait. Implemented inside this crate for
    /// [`crate::handle::WalletHandle`] and
    /// [`crate::handle::PreparedHandle`]; never impl'd by foreign
    /// code, so the public field set of these handles is effectively
    /// sealed.
    pub trait Sealed {}
}

impl sealed::Sealed for crate::handle::WalletHandle {}
impl sealed::Sealed for crate::handle::PreparedHandle {}

pub use error::DartError;
pub use handle::{PreparedHandle, WalletHandle};
pub use status::WalletStatus;
pub use summary::WalletSummary;
