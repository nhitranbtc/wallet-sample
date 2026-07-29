use std::fs;

use chain_core::surface_snapshot::parse_pub_fns;

/// Verify `parse_pub_fns` extracts names only from an inline fixture
/// of the expected FFI surface. This test runs unconditionally because
/// it does not depend on `ffi-bridge/src/api.rs` existing.
#[test]
fn parse_pub_fns_matches_inline_fixture() {
    let fixture = r#"
        pub fn create_wallet() {}
        pub fn restore_wallet_via_native_surface() {}
        pub fn wallet_status() {}
        pub fn list_chains() {}
        pub fn refresh_accounts() {}
        pub fn prepare_native_transfer() {}
        pub fn authenticate_sign_and_broadcast() {}
        pub fn watch_transfer_status() {}
        pub fn get_receive_address() {}
        pub fn lock_wallet() {}
        pub fn remove_wallet() {}
    "#;

    let names = parse_pub_fns(fixture);
    let expected = [
        "create_wallet",
        "restore_wallet_via_native_surface",
        "wallet_status",
        "list_chains",
        "refresh_accounts",
        "prepare_native_transfer",
        "authenticate_sign_and_broadcast",
        "watch_transfer_status",
        "get_receive_address",
        "lock_wallet",
        "remove_wallet",
    ];

    assert_eq!(names.len(), expected.len(), "FFI surface has drifted");

    for name in &expected {
        assert!(
            names.contains(&name.to_string()),
            "missing method {name}"
        );
    }
}

/// Assert that the real `ffi-bridge/src/api.rs` exposes exactly the
/// eleven methods listed in the architecture proof.
///
/// Marked `#[ignore]` because `ffi-bridge/src/api.rs` does not exist
/// yet at this point in the plan (Task 11 lands it). Re-enable the
/// test — by removing the `#[ignore]` attribute — once that crate
/// is in place.
#[test]
#[ignore = "requires ffi-bridge/src/api.rs (Task 11)"]
fn api_rs_only_exposes_eleven_methods() {
    let src = fs::read_to_string("../ffi-bridge/src/api.rs")
        .expect("ffi-bridge/src/api.rs should exist after Task 11");
    let names = parse_pub_fns(&src);

    let expected = [
        "create_wallet",
        "restore_wallet_via_native_surface",
        "wallet_status",
        "list_chains",
        "refresh_accounts",
        "prepare_native_transfer",
        "authenticate_sign_and_broadcast",
        "watch_transfer_status",
        "get_receive_address",
        "lock_wallet",
        "remove_wallet",
    ];

    assert_eq!(
        names.len(),
        expected.len(),
        "FFI surface has drifted (expected {} methods, found {})",
        expected.len(),
        names.len()
    );

    for name in &expected {
        assert!(
            names.contains(&name.to_string()),
            "missing method {name}"
        );
    }
    for found in &names {
        assert!(
            expected.contains(&found.as_str()),
            "unexpected method {found}"
        );
    }
}