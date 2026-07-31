# Developer loop — emulator vs. physical device

The wallet-sample splits its verification surface cleanly in two: **the
host-side Rust + Flutter test suite runs anywhere**, but **the on-device
biometric flow cannot be exercised without real hardware**. This document
captures _what to test where_ so the iteration loop is fast and the
full-flow verification is never skipped.

## The single rule

> **Emulators are fine for unit, widget, and FFI-bridge tests. They are
> NOT fine for any flow that touches `SigningCoordinator`,
> `DestructiveCoordinator`, or `MnemonicSurface`.** Those flows require
> a real device with biometrics enrolled and **not** opted into
> "Always allow" for the wallet app.

The constraint is enforced in code, not in docs:

- `wallet-sample/rust/crates/wallet-orchestration/src/signing_coordinator.rs` —
  `SigningCoordinator` will not consume a `BiometricProof` without real
  biometric attestation.
- `wallet-sample/rust/crates/wallet-orchestration/src/destructive_coordinator.rs` —
  `DestructiveCoordinator` blocks `lock_wallet` / `remove_wallet` paths.
- `wallet-sample/flutter_app/ios/MnemonicSurface.swift` and
  `android/MnemonicSurface.kt` — secure text-entry surfaces only mount
  after a real biometric. The architecture-proof CI also greps these
  two files for `Granted` / `granted: true` / `BiometricProof.granted(`
  patterns and asserts they are absent, so a synthetic-grant factory
  in Swift/Kotlin cannot bypass the Rust `no_synthetic_biometric_proof_path`
  falsifier.

## Threat model

The matrix below defends against five named threats; a future addition
should map to one of them or extend this list:

1. **Emulated biometric bypass** — a developer runs the app on an emulator
   whose biometric prompt is a stub that always returns `Granted`. The
   `SigningCoordinator` / `DestructiveCoordinator` reject emulated proofs,
   but the developer must know this is enforced; otherwise the doc claims
   an empty green CI run proves security.
2. **Synthetic-grant factory in native code** — a Swift/Kotlin helper
   constructs `BiometricProof::Granted` directly, bypassing the Rust
   falsifier. Defended by the `Architecture proof (native)` CI grep.
3. **Test-fixtures leak to release** — `Mnemonic::phrase_for_test` and
   `BiometricProof::granted_for_test` are gated by `test-fixtures`; if a
   release build accidentally enables the feature, the mnemonic phrase
   is reconstructable at runtime. Defended by the `Release-build feature gate`.
4. **Secret material in device logs** — a `println!` or `tracing::info!`
   leaks a mnemonic, WIF, or seed into `adb logcat` / `idevicesyslog`.
   The host-side test suite cannot reach this surface; only the real
   device can. Defended by the `No-secret-material invariant` test
   (developer run + CI device-farm capture).
5. **FFI surface drift** — a careless contributor adds a 12th public
   function to `ffi-bridge/src/api.rs`, breaking the "eleven-method
   frozen surface" contract. Defended by `ffi_surface_is_exactly_eleven_methods`
   and `no_synthetic_biometric_proof_path`.

App-switcher snapshot, FLAG_SECURE / `isSecureTextEntry` enforcement,
and supply-chain compromise are out of scope for this matrix — they are
real concerns but are covered by manual review or external auditors.

## What to test where

| Layer                            | Test                                                                                                                                                            | Where                                          | Why                                                                                                |
| -------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| **Architecture proof**           | `ffi_surface_is_exactly_eleven_methods`                                                                                                                         | Host (cargo test)                              | Reads `ffi-bridge/src/api.rs` and asserts the frozen surface contract; no device APIs needed.      |
| **Architecture proof**           | `no_zeroize_type_in_ffi_api_surface`                                                                                                                            | Host (cargo test)                              | Asserts `Mnemonic`, `WalletSession`, `MnemonicSurface` are absent from the FFI surface.            |
| **Architecture proof**           | `no_synthetic_biometric_proof_path`                                                                                                                             | Host (cargo test)                              | Asserts `BiometricProof::Granted` and the literal `granted: true` are absent from the FFI surface. |
| **Architecture proof (native)** | CI grep over `flutter_app/ios/MnemonicSurface.swift` and `flutter_app/android/MnemonicSurface.kt` for `Granted` / `granted: true` / `BiometricProof.granted(` patterns | CI (host) | Asserts the native-side "no synthetic biometric" equivalent — the Rust falsifier is bypassed by anything constructed in Swift/Kotlin. |
| **Surface snapshot**             | `surface_snapshot::parse_pub_fns_matches_inline_fixture`                                                                                                        | Host (cargo test)                              | Locks the canonical FFI surface against an inline fixture.                                         |
| **Chain registry**               | `registry_list_returns_all_registered_chains`, `registry_round_trips_descriptor`, `empty_registry_lists_nothing`, `registry_register_replaces_existing_adapter` | Host (cargo test)                              | Verifies the 4 chain adapters (Ethereum, Bitcoin, Solana, Tron) are wired via the registry.        |
| **Adapter contracts**            | `chain-{ethereum,bitcoin,solana,tron}/tests/adapter_contract.rs`                                                                                                | Host (cargo test)                              | Per-chain falsifier suite — endpoint validation, address classification, descriptor shape.         |
| **RPC policy**                   | `rpc-client/tests/{endpoint,classifier,retry_policy}.rs`                                                                                                        | Host (cargo test)                              | HTTPS-only, dev_default host allow-list, retry/backoff contract.                                   |
| **Widget UI**                    | `flutter_app/test/widget_tests/*.dart` (6 tests)                                                                                                                | Host (flutter test)                            | Theme tokens, responsive shell, broadcast status, home, review, settings.                          |
| **Controllers**                  | `flutter_app/test/{onboarding,send_draft}_controller_test.dart`                                                                                                 | Host (flutter test)                            | State-machine and form-validity coverage.                                                          |
| **FFI bridge (Dart)**            | `flutter_app/test/{bridge_models,bridge_facade}_test.dart`                                                                                                      | Host (flutter test)                            | Generated-bridge shape and surface parity.                                                         |
| **UI/UX iteration**              | Manual visual review                                                                                                                                            | Emulator (fast)                                | Theme tokens, layouts, navigation. No biometric paths.                                             |
| **Signing flow**                 | `manual-test-release-{1,2}.md` walkthrough                                                                                                                      | Physical device (REQUIRED)                     | `SigningCoordinator` requires real biometric.                                                      |
| **Destructive ops**              | `lock_wallet`, `remove_wallet`                                                                                                                                  | Physical device (REQUIRED)                     | `DestructiveCoordinator` requires real biometric.                                                  |
| **Secure storage**               | Native iOS / Android keystore                                                                                                                                   | Physical device (REQUIRED)                     | Emulator keystore is not the same partition.                                                       |
| **No-secret-material invariant** | `grep -Ei "mnemonic\|seed\|secret_bytes\|0x[0-9a-f]{40}\|tb1[0-9a-z]{6,}\|T[A-Za-z0-9]{33}\|[A-Za-z0-9+/]{40,}={0,2}\|xprv\|xpub"` (developer run) **and** pipe a full device-farm log capture from an integration test through the same grep (CI gate) | Physical device + CI (host) | The host test suite cannot assert this — only the real device's logging surface can. CI captures the device's log via integration test on a device-farm and runs the grep as a gating step. **Pattern coverage**: the regex covers mnemonic / seed / `secret_bytes` literals, hex addresses / Bitcoin testnet / Tron addresses, base64 runs >40 chars, and `xprv`/`xpub` extended keys. It does NOT cover full BIP-39 wordlists (any single 2048-word dictionary entry is a valid pattern), UTF-8 byte dumps of the seed, or partial mnemonic phrases split across lines. Add to the pattern when a new leak vector is identified. |
| **Chain adapter (end-to-end)**  | `manual-test-release-2.md` (Solana) / `manual-test-release-1.md` (other chains) walkthrough                | Physical device (REQUIRED)                     | End-to-end signing on the live device. Step 5 of the iteration loop triggers this for chain-adapter changes. |
| **Release-build feature gate**  | `cargo metadata --no-deps \| jq -e '.resolve.nodes[].features \| .["test-fixtures"] \| not'` | CI (host) | Asserts `test-fixtures` is not in the resolved feature set for any release build — prevents the synthetic-helper leak that the cfg-gate alone does not catch. |

## Recommended iteration loop

1. **Host-side gate** — `cargo test --workspace --all-features --locked` from
   `wallet-sample/rust/`. Includes the three architecture-proof
   falsifier tests. ~5s on a warm cache.
2. **Flutter gate** — `flutter test` from `wallet-sample/`. Includes
   widget, controller, and bridge-facade tests. ~30s.
3. **Lint gate** — `cargo fmt -p <changed-crate> -- --check` and
   `cargo clippy -p <changed-crate> --all-features --all-targets -- -D warnings`.
4. **UI/UX preview** — render the app in an emulator, walk the
   unlocked-screen UX, validate responsive layouts. _Stop here if the
   change touched only theme tokens, screens, or navigation._
5. **Physical-device verification** — if the change touched any
   `*_coordinator*`, `MnemonicSurface*`, secure-storage, or chain
   adapter code, install on a real device and walk through
   `manual-test-release-1.md` (and `-2.md` if Solana is affected).

## Emulator-only commands

```bash
# 1. Build the FFI bridge once
cd wallet-sample/rust
cargo test --workspace --all-features --locked
cd crates/ffi-bridge
flutter_rust_bridge_codegen generate

# 2. Android emulator
$ANDROID_HOME/emulator/emulator -avd <your_avd_name> &
adb devices
flutter run -d emulator-5554

# 3. iOS Simulator (macOS only)
cd wallet-sample
flutter run -d "iPhone 15"
```

## Physical-device commands

```bash
# 1. List connected devices
flutter devices

# 2. Install on a specific device
flutter run -d <device-id>

# 3. Verify no secret material in the device's logging surface
adb logcat | grep -Ei "mnemonic|seed|secret_bytes|0x[0-9a-f]{40}|tb1[0-9a-z]{6,}|T[A-Za-z0-9]{33}|[A-Za-z0-9+/]{40,}={0,2}|xprv|xpub"
# Should produce no matches other than comments / type names. If a match is found:
#   - capture the file/line + raw output
#   - notify #wallet-security (or your team's equivalent)
#   - treat the affected mnemonic as compromised — rotate + scrub device log
#   - block the PR until the leak is rooted in the source

# 4. iOS equivalent (real device — requires `libimobiledevice` for `idevicesyslog`)
idevicesyslog -u <udid> --predicate 'process == "Runner"' \
  | grep -Ei "mnemonic|seed|secret_bytes|0x[0-9a-f]{40}|tb1[0-9a-z]{6,}|T[A-Za-z0-9]{33}|[A-Za-z0-9+/]{40,}={0,2}|xprv|xpub"
# Same incident-response steps as Android above.
```

## Common pitfalls

- **"It works on the emulator"** is not a verification claim for
  anything past the unlock screen. The emulated biometric prompt is a
  stub that always returns `Granted`; the production coordinator
  rejects it.
- The `test-fixtures` feature on `keystore` (and `secure-storage`)
  exposes synthetic helpers like `Mnemonic::phrase_for_test` and
  `BiometricProof::granted_for_test`. These are gated behind
  `#[cfg(any(test, feature = "test-fixtures"))]` so production builds
  compile them out entirely. Do not flip the feature flag on a release
  build — it leaks the mnemonic phrase into the runtime. **CI gate**:
  `cargo metadata --no-deps --format-version 1 | jq -e
  '.resolve.nodes[].features | .["test-fixtures"] | not'` must pass
  for any release profile build.
- `flutter_rust_bridge_codegen generate` must be re-run after any
  change to `wallet-sample/rust/crates/ffi-bridge/src/api.rs` or any
  Rust signature the bridge exposes. The host-side cargo test suite
  will fail loudly if the generated Dart bindings are stale.
- `app-switcher snapshot` is a real-device concern, not an emulator
  one. The emulated app-switcher does not exercise the
  `FLAG_SECURE` (Android) or `isSecureTextEntry` (iOS) intercept.

## When to update this doc

- A new adapter lands in `chain-{name}/` and registers via
  `chain-core/src/registry.rs` → add a row to the _Adapter contracts_
  line.
- A new falsifier test is added to `chain-core/tests/architecture_proof.rs`
  → add a row to the _Architecture proof_ lines.
- A `manual-test-release-N.md` lands → add a corresponding
  _Signing flow_ / _Destructive ops_ row.
- The emulator ban is lifted for a specific flow (would require a
  coordinator change) → split the row out and add an `Emulator ok`
  note.
