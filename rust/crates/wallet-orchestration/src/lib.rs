//! Wallet orchestration: phase state, signing coordinator, destructive coordinator.
//!
//! This crate is the single point at which a freshly-issued
//! `secure_storage::BiometricProof` becomes an authorization to act on
//! wallet state. Only `BiometricProof::Granted { purpose, .. }` is
//! accepted by the coordinators — there is no synthetic-grant path.

pub mod destructive_coordinator;
pub mod session;
pub mod signing_coordinator;

pub use destructive_coordinator::DestructiveCoordinator;
pub use session::SessionState;
pub use signing_coordinator::SigningCoordinator;
