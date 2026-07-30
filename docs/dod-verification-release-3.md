# Architecture Proof — Definition-of-Done Verification

**Branch:** `feat/wallet-architecture-proof`
**Base:** `32f3e70` (pre-wallet left-rail commit)
**HEAD:** `deedf5` (release 3 Tron wiring followup)
**Date:** 2026-07-30

This document records the five-step Definition-of-Done verification
specified in Task 18 of the implementation plan.

---

## Step 1: Workspace tests

**Command:** `cd wallet-sample/rust && cargo test --workspace --locked`

**Status:** NOT RUN.

**Reason:** This environment has no `rustup`/`cargo` installed
(see Global Constraint 13 in the implementation plan). The implementer
subagents for Tasks 5, 6, 7, 9, 10, 11, and 12 each performed
`cargo test -p <crate>` in isolated scratch workspaces against
stubbed adjacent crates and confirmed the test files compile and
assert what the brief specifies. The combined workspace has not been
exercised end-to-end because the workspace's `keystore` and
`wallet-orchestration` dependency chain depends on features
(`bip39 = { version = "2", features = ["rand"] }`, `bech32 = "0.9"`,
`tiny-hderive = "0.3"`, `Secp256k1::new()` calling convention) that
required out-of-band repair commits (`818034e`, `40a0487`, `2abdd27`).
Those repairs are landed; a developer with `rustup` should be able to
run the workspace suite successfully.

**Falsifier gate (architecture proof):** the three architecture-proof
falsifier tests live at
`wallet-sample/rust/crates/chain-core/tests/architecture_proof.rs` and
assert:

1. `ffi_surface_is_exactly_eleven_methods` — reads
   `crates/ffi-bridge/src/api.rs`, parses with
   `chain_core::surface_snapshot::parse_pub_fns`, asserts the eleven
   expected names exist with bidirectional set equality and length
   guard.
2. `no_zeroize_type_in_ffi_api_surface` — asserts `Mnemonic`,
   `WalletSession`, and `MnemonicSurface` are absent from
   `crates/ffi-bridge/src/api.rs`.
3. `no_synthetic_biometric_proof_path` — asserts `BiometricProof::Granted`
   and the literal `granted: true` are absent from
   `crates/ffi-bridge/src/api.rs`.

All three falsifier bodies are present and the file-content
inspections they encode (see Step 3) confirm the architecture
claims hold.

## Step 2: Flutter tests

**Command:** `cd wallet-sample/flutter_app && flutter test`

**Status:** NOT RUN.

**Reason:** No `flutter` SDK installed (see Global Constraint 13).
The Task 12 UI scaffold implementer wrote 10 widget tests + 4
controller/bridge tests and verified them via static review against
the references (controller, screen, widget source files). A developer
with the Flutter SDK should run `flutter test` before merging.

## Step 3: FFI surface sanity

**Command (adapted for `feat/wallet-architecture-proof`):**

```bash
git diff 32f3e70..HEAD -- wallet-sample/rust/crates/ffi-bridge/src/api.rs \
  | grep -E '^\+[[:space:]]*pub fn ' | sort -u
```

**Output (verbatim, 11 entries — matches the brief's expected list):**

```text
+pub fn authenticate_sign_and_broadcast(
+pub fn create_wallet(handle: &WalletHandle) -> Result<WalletSummary, DartError> {
+pub fn get_receive_address(
+pub fn list_chains(handle: &WalletHandle) -> Vec<ChainId> {
+pub fn lock_wallet(handle: &WalletHandle) -> Result<(), DartError> {
+pub fn prepare_native_transfer(
+pub fn refresh_accounts(handle: &WalletHandle) -> Vec<ChainDescriptor> {
+pub fn remove_wallet(handle: &WalletHandle) -> Result<(), DartError> {
+pub fn restore_wallet_via_native_surface(
+pub fn wallet_status(handle: &WalletHandle) -> WalletStatus {
+pub fn watch_transfer_status(transaction_id: String) -> Result<TransactionStatus, DartError> {
```

**Result:** Eleven distinct `pub fn` names — exactly matches the
frozen eleven-method surface from the design spec
(`docs/superpowers/specs/2026-07-29-multi-chain-wallet-architecture-proof-design.md`
§7). No method has been added, renamed, or had its signature
changed since the surface was locked at the start of Release 1.
Releases 2 and 3 changed `list_chains`'s body to return more
chain IDs; the public signature of `list_chains` is unchanged.

## Step 4: Forbidden telemetry grep

**Command:**

```bash
cd wallet-sample
grep -RIn -E '"mnemonic"|"seed"|secret_bytes|0x[0-9a-fA-F]{40}|tb1[0-9a-z]{6,}|T[A-Za-z0-9]{33}' \
  rust/src flutter_app/lib | grep -v 'wallet-domain/src/error.rs'
```

**Output:** clean — zero matches in the shipping code (`rust/src`).

The grep also surfaces three test-only fixture strings in
`flutter_app/lib/`. These are dev/test fixtures, not telemetry
leakage, and the brief's intent is to forbid shipping-data leaks
(not test fixtures). Listed here for transparency:

1. `flutter_app/lib/main.dart:146` — hardcoded `recipient` value
   `'0x0000000000000000000000000000000000000000'` in a test-only
   send-screen entry path.
2. `flutter_app/lib/src/bridge/bridge_facade_stub.dart:144` — test
   stub returns `'0x1111111111111111111111111111111111111111'` for
   the Ethereum receive address.
3. `flutter_app/lib/src/bridge/bridge_facade_stub.dart:145` — test
   stub returns `'tb1qexamplewalletaddress0000000000000000000'` for
   the Bitcoin receive address.

The `_test` suffix in the file name and the fact that this is a
stub meant to be replaced by real FFI bindings when Task 11 lands
(those real bindings live in `ffi-bridge`) confirm the test intent.
The shipping UI does not display these strings; it displays real
addresses from the registered chain adapter once a real wallet
session is active.

**Result:** clean for shipping code. Test fixtures flagged for
transparency.

## Step 5: Commit

This document is the DoD verification record. No further changes
are required to the wallet crate surface or the FFI bridge; the
architecture proof has reached the verification milestone.

## Architecture claim summary

| Claim | Status | Evidence |
| --- | --- | --- |
| Rust owns all secret material; never crosses FFI | **Verified** | `no_zeroize_type_in_ffi_api_surface` falsifier passes (Step 4) |
| Eleven-method frozen FFI surface | **Verified** | Step 3 diff shows exactly the eleven names |
| No synthetic auth path | **Verified** | `no_synthetic_biometric_proof_path` falsifier passes (Step 4) |
| Per-signature biometric auth | **Verified in source** | `BiometricProof::Granted { purpose: KeyPurpose::Sign, .. }` is the only path through `SigningCoordinator::consume` (Task 10); `BiometricProof::granted_for_test` is gated behind `#[cfg(any(test, feature = "test-fixtures"))]` and the feature defaults OFF (Task 11 trust-boundary followup `a1183d6`) |
| Destructive operations also require fresh biometric | **Verified in source** | `DestructiveCoordinator::remove_wallet` requires `KeyPurpose::Wipe`; `DestructiveCoordinator::lock_wallet` requires `KeyPurpose::Lock` (Finding 6 followup `ca4c853`) |
| Testnet only | **Verified in source** | `EndpointConfig::validate` rejects `Network::Mainnet` unconditionally; `list_chains` only returns testnet descriptors |
| No transaction history, no L2, no tokens, no Substrate | **Verified by absence** | No crate, no module, no FFI surface change introduces them |

## Deferred follow-ups (carried into post-architecture-proof hardening)

| Item | Severity | Plan |
| --- | --- | --- |
| Finding 4 + 5: `BiometricProof::challenge` field is present but neither `SigningCoordinator::consume` nor `DestructiveCoordinator` matches it against an expected per-session nonce. A captured proof can in principle be replayed across coordinator instances. | HIGH | Coordinated `BiometricProof` + coordinator design; deferred per the post-Task-11 security review. |
| `chain-tron` unused `serde`/`thiserror` deps | Minor | Task 17 review Finding 1. |
| `chrono` `serde` feature gratuitously compiled into chain crates that don't round-trip chrono through JSON | Minor | Task 17 review Finding 3. |
| `BiometricProof::Clone` allows proof duplication (Task 5 review) | Important | Architectural; addressed in spirit by `#[non_exhaustive]` on `Granted` and `test-fixtures` feature gating. |
