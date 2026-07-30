# Wallet Sample — Architecture Proof

Ethereum (Sepolia) and Bitcoin (testnet). This is the architecture proof; not a production wallet.

## Build

```bash
cd rust
cargo test --workspace --locked
cd ..
flutter create --project-name wallet_sample flutter_app # if not yet created
cd flutter_app
flutter pub get
cd ../rust/crates/ffi-bridge
flutter_rust_bridge_codegen generate
cd ../../..
flutter run
```

## Endpoints

Configure Sepolia RPC and Bitcoin Esplora URLs in `flutter_app/lib/src/config/endpoints.dart`. Only hosts in `ProviderPolicy::dev_default` are accepted; HTTPS only.

## Release 1 gate

- All workspace tests pass under `cargo test --workspace --locked`.
- The three architecture-proof falsifier tests pass.
- Manual testnet verification captured in `docs/manual-test-release-1.md`.
- iOS and Android secure storage verified on physical devices.
- No secret material appears in `flutter logs`, `adb logcat`, or Xcode console.

This build is not suitable for mainnet custody.
