import 'package:flutter_test/flutter_test.dart';
import 'package:wallet_sample/src/bridge/bridge_facade_stub.dart';

void main() {
  test('all eleven stub methods return well-formed defaults', () async {
    final bridge = BridgeFacade();

    final created = await bridge.createWallet();
    final restored = await bridge.restoreWalletViaNativeSurface();
    final status = bridge.walletStatus();
    final chains = bridge.listChains();
    final accounts = bridge.refreshAccounts();
    final prepared = await bridge.prepareNativeTransfer(
      chain: ChainId.ethereum,
      recipient: '0x0000000000000000000000000000000000000001',
      amount: BigInt.one,
    );
    final hash = await bridge.authenticateSignAndBroadcast(prepared: prepared);
    final transactionStatus = bridge.watchTransferStatus(hash);
    final address = bridge.getReceiveAddress(ChainId.bitcoin);
    await bridge.lockWallet();
    await bridge.removeWallet();

    expect(created.handle, isA<WalletHandle>());
    expect(restored.enabledChains, isNotEmpty);
    expect(status.initialized, isTrue);
    expect(status.locked, isFalse);
    expect(status.enabled_chains, hasLength(2));
    expect(status.last_sync_at, isNull);
    expect(chains, [ChainId.ethereum, ChainId.bitcoin]);
    expect(accounts, hasLength(2));
    expect(prepared, isA<PreparedHandle>());
    expect(hash, startsWith('0x'));
    expect(transactionStatus, TransactionStatus.Pending);
    expect(address, isNotEmpty);
  });
}
