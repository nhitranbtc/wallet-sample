//! Chain-adapter layer for the multi-chain wallet proof.
//!
//! [`ChainAdapter`] is the object-safe contract every chain
//! implementation must satisfy; [`ChainRegistry`] is the in-process
//! index of installed adapters that the FFI bridge and the
//! orchestration layer both read from.

pub mod adapter;
pub mod fee;
pub mod prepared;
pub mod registry;
pub mod surface_snapshot;

pub use adapter::ChainAdapter;
pub use prepared::PreparedPayload;
pub use registry::ChainRegistry;
pub use wallet_domain::fee::ResourceSummary;