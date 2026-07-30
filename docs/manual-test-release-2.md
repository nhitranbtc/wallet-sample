# Manual testnet verification — Release 2 (Solana devnet)

This is a hands-on checklist that **complements** the automated
`cargo test --workspace` contract for the Release-2 Solana
adapter. It is the analog of `manual-test-release-1.md` (Ethereum
Sepolia + Bitcoin testnet) and runs on a real iOS or Android device
with biometrics enrolled. Do not run any of this on an emulator —
biometric hardware is required for the destructive and signing
flows.

The Release-2 adapter lives in `wallet-sample/rust/crates/chain-solana/`,
registered in `wallet-sample/rust/crates/ffi-bridge/src/services/wallet_status.rs::list_chains`
so that `list_chains` returns `vec![Ethereum, Bitcoin, Solana]`.
The adapter accepts a `SolanaConfig { network, endpoint, rpc_url }`,
validates `endpoint` against `rpc_client::EndpointConfig::validate`
(HTTPS-only, devnet-only, host allow-list), and builds a
`PreparedPayload::Sol { blockhash: [0u8; 32] }` with a zero-byte
placeholder blockhash (real impl will fetch via
`solana_rpc_client::RpcClient::get_latest_blockhash`).

## ⚠️ Release 2 broadcast wiring (deferred)

**Steps 7–8 below cannot pass on this build and are marked
`BLOCKED`.** The Solana adapter ships validation and preparation
only; the signing and transport path is not wired:

- `chain-solana` declares `solana-signer` and `solana-rpc-client`
  in `Cargo.toml` but calls neither. Each call site that will use
  them carries a `FIXME(Release 2 broadcast wiring)` comment in
  `crates/chain-solana/src/adapter.rs`.
- `prepare_transfer` embeds a zero-byte blockhash placeholder
  instead of a fetched one, so the prepared payload could not be
  accepted by devnet even if it were signed.
- `SolanaAdapter::broadcast` returns an empty `TransactionId`, and
  `transaction_status` is hard-coded to `Pending` — a poll loop
  over it would never terminate.
- `ffi-bridge`'s `authenticate_sign_and_broadcast`
  (`crates/ffi-bridge/src/services/signing.rs`) returns
  `ErrorCategory::Broadcast` for `ChainId::Solana` rather than
  reaching the adapter at all.

Until that wiring lands, treat steps 1–6 and 9–10 as the live
Release-2 checklist. Do **not** file a bug for a failed step 7 or
8; the expected observation there is the `Broadcast` error from
the FFI layer.

## Preconditions

- **Device**: iOS 16+ or Android 12+ with Face ID / Touch ID /
  fingerprint enrolled and **not** opted into "Always allow" for
  the wallet app.
- **Build**: install the Release-2 build of `flutter_app/` from
  the `wallet-sample/rust` workspace compiled with
  `flutter_rust_bridge`.
- **Testnet faucet** (test tokens only — never send mainnet funds):
  - **SOL devnet** — `https://faucet.solana.com` (select "Devnet")
    or `https://solfaucet.com`.
- **Explorer**:
  - Solana devnet: `https://explorer.solana.com?cluster=devnet`.

## Per-chain steps

### Solana (devnet)

1. Launch the wallet. Confirm a real OS biometric prompt fires for
   the initial unlock.
2. Tap **Create wallet** (or **Restore** if you already have a
   Release-2 test vector). A new biometric prompt must fire — the
   `restore_wallet_via_native_surface` path is gated on the
   platform secure-storage shim.
3. Open the **Chain list** screen. Confirm the chain list now
   contains three entries — **Ethereum (Sepolia)**, **Bitcoin
   (testnet)**, and **Solana (devnet)** — in that order. The
   three-entry set is the surface contract that
   `ffi-bridge::list_chains` pins; if Solana is missing the
   registry was not updated.
4. Tap **Receive** for Solana. Confirm the receive address is a
   base58 string of length 32 bytes (i.e. a real ed25519 pubkey,
   not the 44-character `11111…` System Program placeholder).
5. From an external Solana devnet wallet, send **0.001 SOL** to
   that address from the devnet faucet. Wait for one confirmation
   on `https://explorer.solana.com?cluster=devnet`.
6. Tap **Send** → **Solana**. Enter the destination address (any
   other devnet address you control) and an amount of **0.0005
   SOL**. Confirm the app rejects the System Program address
   `11111111111111111111111111111111` as an invalid destination —
   it is an all-zero (identity) ed25519 point and
   `prepare_transfer` refuses it.
7. **BLOCKED — Release 2 broadcast wiring (deferred).** Tap
   **Review**. A biometric prompt must fire for
   `authenticate_sign_and_broadcast`. **Do not** dismiss it
   without authenticating — the `SigningCoordinator` will refuse
   to consume the proof otherwise. On this build the call then
   fails with an `ErrorCategory::Broadcast` error instead of
   signing; that is the expected result, not a bug.
8. **BLOCKED — Release 2 broadcast wiring (deferred).** There is
   no broadcast on this build, so no pending status and no devnet
   signature to poll for. Re-enable this step (and step 7) once
   `solana-signer` / `solana-rpc-client` are wired through
   `SolanaAdapter::broadcast` and the `ffi-bridge` Solana arm.
9. Tap **Lock wallet**. A fresh biometric prompt must fire
   (lock is a destructive coordinator path). Verify the wallet
   UI now shows the locked state.
10. Tap **Remove wallet**. A fresh biometric prompt must fire
    (wipe is a destructive coordinator path). Confirm the wallet
    reports an empty state.

## Re-running the Release-1 checklist

The Release-2 Solana adapter addition must not regress any
Release-1 path. Re-run the full `manual-test-release-1.md`
checklist (Ethereum Sepolia + Bitcoin testnet) and tick every box
on the same device with the same build. In particular:

- `list_chains` must still report Ethereum and Bitcoin (in that
  order, before Solana).
- The Bitcoin address returned by **Receive → Bitcoin** must
  still start with `tb1` (bech32 testnet).
- The Sepolia receive address returned by **Receive → Ethereum**
  must still start with `0x`.

## Verification checklist

For every step above, tick the box only after the behavior is
observed. If any box cannot be ticked, file a bug with the device
model, OS build, and `flutter logs` / `adb logcat` / Xcode console
output — **never paste the mnemonic or any signed payload** in
the report.

- [ ] Recovery UI never appears in the iOS / Android app-switcher
      snapshot. (Switch to the home screen mid-recovery and verify
      the recovery prompt does not render in the snapshot, only the
      platform's lock screen.)
- [ ] No secret material in `flutter logs`, `adb logcat`, or the
      Xcode console. Specifically: no mnemonic phrases, no ed25519
      secret bytes, no derived Solana keypair material, no
      signed-transaction payloads, no `BiometricProof` values.
- [ ] `list_chains` returns exactly
      `[Ethereum, Bitcoin, Solana]` in that order on the same
      device build.
- [ ] Solana transaction confirmed on
      `https://explorer.solana.com?cluster=devnet` (≥ 1
      confirmation).
- [ ] Each Solana signature required a fresh biometric prompt —
      i.e. you could not queue multiple
      `authenticate_sign_and_broadcast` calls back-to-back without
      re-prompting.
- [ ] Each destructive operation (`lock_wallet`, `remove_wallet`)
      required a fresh biometric prompt.
- [ ] No Release-1 regression: Sepolia and Bitcoin testnet paths
      pass the `manual-test-release-1.md` checklist unchanged.
