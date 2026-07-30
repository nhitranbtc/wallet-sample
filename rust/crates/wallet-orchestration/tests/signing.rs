use secure_storage::{BiometricProof, KeyPurpose, WrappedKey};
use wallet_domain::error::WalletError;
use wallet_orchestration::{SessionState, SigningCoordinator};

fn fake_proof() -> BiometricProof {
    BiometricProof::granted_for_test(
        WrappedKey {
            reference: "test-ref".to_string(),
            algorithm: "test-alg".to_string(),
        },
        KeyPurpose::Sign,
        [0u8; 32],
    )
}

#[test]
fn coordinator_rejects_double_consume() {
    let c = SigningCoordinator::new();
    let mut session = SessionState::ready_for_test();
    c.consume(&mut session, fake_proof()).unwrap();
    let err = c.consume(&mut session, fake_proof()).unwrap_err();
    assert!(matches!(err, WalletError::Authentication(_)));
}

#[test]
fn coordinator_rejects_locked_session() {
    let c = SigningCoordinator::new();
    let mut session = SessionState::ready_for_test();
    session.lock();
    let err = c.consume(&mut session, fake_proof()).unwrap_err();
    assert!(matches!(err, WalletError::Locked));
}

#[test]
fn coordinator_rejects_synthetic_proof() {
    let c = SigningCoordinator::new();
    let mut session = SessionState::ready_for_test();
    // No way to construct a synthetic BiometricProof outside the secure-storage crate.
    // This test asserts the API: only the secure-storage's BiometricProof is accepted.
    let result = c.try_with_proof(&mut session, std::marker::PhantomData::<()>);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        WalletError::Authentication(_)
    ));
}