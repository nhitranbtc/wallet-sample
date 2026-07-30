use alloy::signers::local::PrivateKeySigner;
use keystore::Derive;
use wallet_domain::error::WalletError;

/// Builds an alloy signer from the derived account-zero EVM key.
///
/// # Task 10 boundary
///
/// The on-device signing wiring lives in **Task 10
/// (`wallet-orchestration`)**, which owns the unlocked
/// [`keystore::WalletSession`] and is the only layer allowed to hold
/// secret material. Task 10 completes this function by constructing
/// `PrivateKeySigner` over the derived secret bytes; until then the
/// call site does not exist and this body is intentionally
/// `unimplemented!()` rather than a silently-wrong stub signer.
pub fn build_signer(session: &impl Derive) -> Result<PrivateKeySigner, WalletError> {
    let _sk = session.derive_evm_key()?;
    unimplemented!(
        "Task 10 (wallet-orchestration) wires PrivateKeySigner over the derived EVM secret"
    )
}
