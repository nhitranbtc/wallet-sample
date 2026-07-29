use chrono::{Duration, Utc};
use wallet_domain::{
    account::{AccountRef, AddressDisplay, ChainId, Network},
    amount::Amount,
    broadcast::{TransactionId, TransactionStatus},
    error::{ChainError, ErrorCategory},
    snapshot::SnapshotStatus,
    transfer::{FeeEstimate, PreparedTransfer, TransferRequest},
};

fn account() -> AccountRef {
    AccountRef { chain: ChainId::Ethereum, network: Network::Testnet, index: 0 }
}

#[test]
fn prepared_transfer_rejects_account_mismatch() {
    let now = Utc::now();
    let prep = PreparedTransfer {
        preparation_id: "p1".into(),
        source: account(),
        destination: AddressDisplay("0xabc".into()),
        amount: Amount(1),
        fee: FeeEstimate::native_gas(Amount(2), Amount(3)),
        network: Network::Testnet,
        expires_at: now + Duration::minutes(5),
        status: SnapshotStatus::Fresh,
        payload: wallet_domain::prepared::PreparedPayload::Eth,
    };
    let other = AccountRef { chain: ChainId::Bitcoin, network: Network::Testnet, index: 0 };
    let err = prep.validate_fresh(&other, Network::Testnet).unwrap_err();
    assert_eq!(err.category(), ErrorCategory::ChainState);
}

#[test]
fn prepared_transfer_rejects_expiry() {
    let now = Utc::now();
    let prep = PreparedTransfer {
        preparation_id: "p2".into(),
        source: account(),
        destination: AddressDisplay("0xabc".into()),
        amount: Amount(1),
        fee: FeeEstimate::native_gas(Amount(2), Amount(3)),
        network: Network::Testnet,
        expires_at: now - Duration::seconds(1),
        status: SnapshotStatus::Fresh,
        payload: wallet_domain::prepared::PreparedPayload::Eth,
    };
    assert!(matches!(prep.validate_fresh(&account(), Network::Testnet), Err(ChainError::ChainState(_))));
}

#[test]
fn chain_error_categories_are_stable() {
    assert_eq!(ChainError::Input("x".into()).category(), ErrorCategory::Input);
    assert_eq!(ChainError::Configuration("x".into()).category(), ErrorCategory::Configuration);
    assert_eq!(ChainError::Connectivity("x".into()).category(), ErrorCategory::Connectivity);
    assert_eq!(ChainError::ChainState("x".into()).category(), ErrorCategory::ChainState);
    assert_eq!(ChainError::Authorization("x".into()).category(), ErrorCategory::Authorization);
    assert_eq!(ChainError::Broadcast("x".into()).category(), ErrorCategory::Broadcast);
    assert_eq!(ChainError::Internal("x".into()).category(), ErrorCategory::Internal);
}

#[test]
fn amount_checked_add_overflows_cleanly() {
    assert_eq!(Amount(7).checked_add(Amount(3)), Some(Amount(10)));
    assert_eq!(Amount(u128::MAX).checked_add(Amount(1)), None);
}

#[test]
fn transfer_request_round_trip_serializes() {
    let req = TransferRequest {
        source: account(),
        destination: AddressDisplay("0xdeadbeef".into()),
        amount: Amount(42),
    };
    let json = serde_json::to_string(&req).unwrap();
    let back: TransferRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(req, back);
    let _ = TransactionId("0xabc".to_string());
    let _ = TransactionStatus::Pending;
}
