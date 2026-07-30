//! Architecture-proof falsifier contract.
//!
//! These three tests gate the wallet's architecture claims at the
//! workspace level. They are deliberately written as **falsifiers**:
//! each test must FAIL if the architecture claim it is supposed to
//! uphold is violated.
//!
//! The companion to this file is
//! `crates/ffi-bridge/tests/surface_snapshot_test.rs`, which asserts
//! the same surface from inside the `ffi-bridge` crate. The three
//! tests here additionally run at the workspace-root integration
//! level so a single `cargo test --workspace` invocation fails fast
//! on any architecture drift, regardless of which crate owns the
//! offender.
//!
//! Failure semantics:
//! 1. `ffi_surface_is_exactly_eleven_methods` — fails if the FFI
//!    bridge adds, removes, or renames a `pub fn`. Bidirectional set
//!    equality + length guard.
//! 2. `no_zeroize_type_in_ffi_api_surface` — fails if any of the
//!    zeroize-bearing types (`Mnemonic`, `WalletSession`,
//!    `MnemonicSurface`) are referenced from
//!    `crates/ffi-bridge/src/api.rs`. Their `Drop` impls must not
//!    run from the FFI surface.
//! 3. `no_synthetic_biometric_proof_path` — fails if the FFI bridge
//!    can manufacture a `BiometricProof::Granted` value or a
//!    `granted: true` literal. The proof must always be fetched
//!    from the platform secure-storage shim.

use chain_core::surface_snapshot::parse_pub_fns;

/// Resolve `crates/ffi-bridge/src/api.rs` from inside this crate's
/// integration-test process. We use `CARGO_MANIFEST_DIR` (set by
/// cargo at test runtime) plus a relative walk so the path is
/// independent of the test runner's working directory.
///
/// Layout on disk:
///   <workspace>/Cargo.toml
///   <workspace>/crates/chain-core/Cargo.toml   <- CARGO_MANIFEST_DIR
///   <workspace>/crates/ffi-bridge/src/api.rs   <- the spec under test
fn ffi_api_rs_path() -> std::path::PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR set by cargo at integration-test runtime");
    std::path::PathBuf::from(manifest_dir)
        .join("..")
        .join("ffi-bridge")
        .join("src")
        .join("api.rs")
}

#[test]
fn ffi_surface_is_exactly_eleven_methods() {
    let api_rs_path = ffi_api_rs_path();
    let src = std::fs::read_to_string(&api_rs_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", api_rs_path.display()));

    let names = parse_pub_fns(&src);
    let expected: &[&str] = &[
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

    // Length guard: catches both "added one" and "removed one" in
    // one assertion, before the set-equality checks below give a
    // more diagnostic message.
    assert_eq!(
        names.len(),
        expected.len(),
        "ffi-bridge surface length drifted: expected {} pub fns, found {}: {names:?}",
        expected.len(),
        names.len(),
    );

    // Every expected name is present (catches renames + removals).
    for want in expected {
        assert!(
            names.iter().any(|n| n == want),
            "missing expected pub fn `{want}` in ffi-bridge/src/api.rs; got: {names:?}",
        );
    }

    // Every actual name is expected (catches additions + typos).
    for found in &names {
        assert!(
            expected.iter().any(|e| *e == found),
            "unexpected pub fn `{found}` in ffi-bridge/src/api.rs; got: {names:?}",
        );
    }
}

#[test]
fn no_zeroize_type_in_ffi_api_surface() {
    let api_rs_path = ffi_api_rs_path();
    let src = std::fs::read_to_string(&api_rs_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", api_rs_path.display()));

    let forbidden = ["Mnemonic", "WalletSession", "MnemonicSurface"];
    for needle in forbidden {
        assert!(
            !src.contains(needle),
            "zeroize-bearing type `{needle}` appears in ffi-bridge/src/api.rs; \
             the FFI surface must not reference types whose `Drop` impl \
             would run there",
        );
    }
}

#[test]
fn no_synthetic_biometric_proof_path() {
    let api_rs_path = ffi_api_rs_path();
    let src = std::fs::read_to_string(&api_rs_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", api_rs_path.display()));

    // Direct construction of a granted proof is forbidden: the only
    // valid source for `BiometricProof` is the platform secure-storage
    // shim under `secure-storage/src/platform/*`.
    assert!(
        !src.contains("BiometricProof::Granted"),
        "ffi-bridge must not construct `BiometricProof::Granted` directly; \
         proofs are only valid when fetched from platform secure-storage",
    );

    // A literal `granted: true` would be a back-door equivalent of
    // the above — flag it as the same architecture violation.
    assert!(
        !src.contains("granted: true"),
        "no synthetic `granted: true` literal is allowed in ffi-bridge; \
         the biometric gate must come from the OS prompt",
    );
}
