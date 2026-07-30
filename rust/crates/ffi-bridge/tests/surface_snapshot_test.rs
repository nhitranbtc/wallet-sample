//! Frozen-surface snapshot test for `api.rs`.
//!
//! The eleven-method FFI surface in `src/api.rs` is hand-picked and
//! must not drift — every adapter-layer change must keep that surface
//! intact. This test parses `src/api.rs` via
//! `chain_core::surface_snapshot::parse_pub_fns` and asserts that
//! exactly the expected method names appear, no more, no less.
//!
//! The test re-reads its own source file at runtime because
//! `parse_pub_fns` operates on the raw `pub fn` text and `api.rs`
//! itself is the only "spec" we have — the list is encoded there.
//! The expected list is duplicated inline below so a typo in either
//! location fails this test loudly.

use chain_core::surface_snapshot::parse_pub_fns;

const EXPECTED: &[&str] = &[
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

#[test]
fn api_rs_only_exposes_eleven_methods() {
    // The test lives at `crates/ffi-bridge/tests/surface_snapshot_test.rs`.
    // `src/api.rs` is one directory up from this test file.
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR set by cargo at test runtime");
    let api_rs_path = format!("{manifest_dir}/src/api.rs");
    let src = std::fs::read_to_string(&api_rs_path)
        .unwrap_or_else(|e| panic!("read {api_rs_path}: {e}"));
    let names = parse_pub_fns(&src);

    for expected in EXPECTED {
        assert!(
            names.contains(&expected.to_string()),
            "missing expected pub fn `{expected}` from api.rs; got: {names:?}",
        );
    }
    for name in &names {
        assert!(
            EXPECTED.contains(&name.as_str()),
            "unexpected pub fn `{name}` found in api.rs; got: {names:?}",
        );
    }
    assert_eq!(
        names.len(),
        EXPECTED.len(),
        "expected exactly {} pub fns in api.rs, found {}: {names:?}",
        EXPECTED.len(),
        names.len(),
    );
}
