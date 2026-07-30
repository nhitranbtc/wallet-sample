# Manual testnet verification — Release 3 (Tron Shasta)

This is a hands-on checklist that **complements** the automated
`cargo test --workspace` contract for the Release-3 Tron
adapter. It is the analog of `manual-test-release-2.md` (Solana
devnet) and `manual-test-release-1.md` (Ethereum Sepolia +
Bitcoin testnet) and runs on a real iOS or Android device with
biometrics enrolled. Do not run any of this on an emulator —
biometric hardware is required for the destructive and signing
flows.

The Release-3 adapter lives in `wallet-sample/rust/crates/chain-tron/`,
registered in `wallet-sample/rust/crates/ffi-bridge/src/services/wallet_status.rs::list_chains`
so that `list_chains` returns
`vec![Ethereum, Bitcoin, Solana, Tron]`. The adapter accepts a
`TronConfig { network, endpoint, chain_id }` (Shasta chain id
`2494104990`), validates `endpoint` against
`rpc_client::EndpointConfig::validate` (HTTPS-only,
testnet-only, host allow-list for `api.shasta.trongrid.io`),
and builds a `PreparedPayload::Tron { ref_block_bytes: [0, 0],
ref_block_hash: [0; 8] }` with a zero-byte placeholder block
reference (a real implementation fetches it via
`trongrid::wallet/getnowblock`).

## Preconditions

- **Device**: iOS 16+ or Android 12+ with Face ID / Touch ID /
  fingerprint enrolled and **not** opted into "Always allow" for
  the wallet app.
- **Build**: install the Release-3 build of `flutter_app/` from
  the `wallet-sample/rust` workspace compiled with
  `flutter_rust_bridge`.
- **Testnet faucet** (test tokens only — never send mainnet funds):
  - **TRX Shasta** — `https://www.trongrid.io/faucet` (select
    "Shasta") or community-run Shasta faucets surfaced in the
    Tron developer documentation. Trx is required because
    `prepare_transfer` reserves the bandwidth/energy budget
    (`ResourceSummary::TronBandwidthAndEnergy`) the proof uses
    for the architecture contract.
- **Explorer**:
  - Tron Shasta: `https://shasta.tronscan.org`.

## Per-chain steps

### Tron (Shasta)

1. Launch the wallet. Confirm a real OS biometric prompt fires for
   the initial unlock.
2. Tap **Create wallet** (or **Restore** if you already have a
   Release-3 test vector). A new biometric prompt must fire — the
   `restore_wallet_via_native_surface` path is gated on the
   platform secure-storage shim.
3. Open the **Chain list** screen. Confirm the chain list now
   contains four entries — **Ethereum (Sepolia)**, **Bitcoin
   (testnet)**, **Solana (devnet)**, and **Tron (Shasta)** — in
   that order. The four-entry set is the surface contract that
   `ffi-bridge::list_chains` pins; if Tron is missing the
   registry was not updated.
4. Tap **Receive** for Tron. Confirm the receive address is a
   34-character base58 string starting with `T` (a real
   base58check address — `0x41` mainnet prefix + 20-byte
   address + 4-byte double-SHA256 checksum — derived at
   BIP-44 path `m/44'/195'/0'/0/0`, not the
   `TExampleTronAddress…` placeholder).
5. From an external Tron Shasta wallet, send **1 TRX** to that
   address from the Shasta faucet. Wait for one confirmation on
   `https://shasta.tronscan.org`.
6. Tap **Send** → **Tron**. Enter the destination address (any
   other Shasta address you control) and an amount of
   **0.5 TRX**. Confirm `prepare_transfer` builds a
   `PreparedPayload::Tron { ref_block_bytes: [0, 0],
   ref_block_hash: [0; 8] }` with the zero-byte block-reference
   placeholder and a `ResourceSummary::TronBandwidthAndEnergy`
   reservation of `600` bandwidth / `65_000` energy.
7. Tap **Review**. A biometric prompt must fire for
   `authenticate_sign_and_broadcast`. **Do not** dismiss it
   without authenticating — the `SigningCoordinator` will refuse
   to consume the proof otherwise.
8. After broadcast, the app should show a pending status; poll
   until the Shasta explorer shows ≥ 1 confirmation on
   `https://shasta.tronscan.org`.
9. Tap **Lock wallet**. A fresh biometric prompt must fire
   (lock is a destructive coordinator path). Verify the wallet
   UI now shows the locked state.
10. Tap **Remove wallet**. A fresh biometric prompt must fire
    (wipe is a destructive coordinator path). Confirm the wallet
    reports an empty state.

## Re-running the Release-1 and Release-2 checklists

The Release-3 Tron adapter addition must not regress any
Release-1 or Release-2 path. Re-run the full
`manual-test-release-1.md` checklist (Ethereum Sepolia + Bitcoin
testnet) and the full `manual-test-release-2.md` checklist
(Solana devnet) and tick every box on the same device with the
same build. In particular:

- `list_chains` must still report Ethereum, Bitcoin, and Solana
  (in that order, before Tron).
- The Bitcoin address returned by **Receive → Bitcoin** must
  still start with `tb1` (bech32 testnet).
- The Sepolia receive address returned by **Receive → Ethereum**
  must still start with `0x`.
- The Solana devnet receive address returned by **Receive →
  Solana** must still be a 32-byte ed25519 base58 pubkey (not
  the 44-character `11111…` System Program placeholder).

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
      Xcode console. Specifically: no mnemonic phrases, no
      secp256k1 secret bytes, no derived Tron keypair material, no
      signed-transaction payloads, no `BiometricProof` values, no
      base58check `0x41`-prefixed private keys.
- [ ] `list_chains` returns exactly
      `[Ethereum, Bitcoin, Solana, Tron]` in that order on the
      same device build.
- [ ] Tron transaction confirmed on
      `https://shasta.tronscan.org` (≥ 1 confirmation).
- [ ] Each Tron signature required a fresh biometric prompt —
      i.e. you could not queue multiple
      `authenticate_sign_and_broadcast` calls back-to-back without
      re-prompting.
- [ ] Each destructive operation (`lock_wallet`, `remove_wallet`)
      required a fresh biometric prompt.
- [ ] No Release-1 regression: Sepolia and Bitcoin testnet paths
      pass the `manual-test-release-1.md` checklist unchanged.
- [ ] No Release-2 regression: Solana devnet paths pass the
      `manual-test-release-2.md` checklist unchanged.
