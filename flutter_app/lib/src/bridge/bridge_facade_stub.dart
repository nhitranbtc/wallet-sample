// STUB — replaced by the real BridgeFacade once flutter_rust_bridge codegen lands (Task 11). The eleven-method contract is frozen.

class WalletHandle {
  WalletHandle() : _opaque = Object();
  final Object _opaque;
}

class PreparedHandle {
  PreparedHandle() : _opaque = Object();
  final Object _opaque;
}

enum ChainId { ethereum, bitcoin }

enum TransactionStatus { Pending, Confirmed, Failed, Unknown }

class ChainDescriptor {
  const ChainDescriptor({
    required this.id,
    required this.name,
    required this.symbol,
    required this.network,
    required this.isTestnet,
    required this.balance,
  });

  final ChainId id;
  final String name;
  final String symbol;
  final String network;
  final bool isTestnet;
  final String balance;
}

class WalletStatus {
  const WalletStatus({
    required this.initialized,
    required this.locked,
    required this.enabled_chains,
    required this.last_sync_at,
  });

  final bool initialized;
  final bool locked;
  final List<ChainDescriptor> enabled_chains;
  final DateTime? last_sync_at;
}

class WalletSummary {
  const WalletSummary({
    required this.handle,
    required this.displayName,
    required this.enabledChains,
  });

  final WalletHandle handle;
  final String displayName;
  final List<ChainId> enabledChains;
}

class AccountSummary {
  const AccountSummary({
    required this.chain,
    required this.address,
    required this.balance,
  });

  final ChainId chain;
  final String address;
  final String balance;
}

class BridgeException implements Exception {
  const BridgeException(this.message);
  final String message;

  @override
  String toString() => 'BridgeException: $message';
}

class BridgeFacade {
  final WalletHandle _walletHandle = WalletHandle();

  static const ChainDescriptor _ethereum = ChainDescriptor(
    id: ChainId.ethereum,
    name: 'Ethereum',
    symbol: 'ETH',
    network: 'Sepolia testnet',
    isTestnet: true,
    balance: '1.2500',
  );

  static const ChainDescriptor _bitcoin = ChainDescriptor(
    id: ChainId.bitcoin,
    name: 'Bitcoin',
    symbol: 'BTC',
    network: 'Bitcoin testnet',
    isTestnet: true,
    balance: '0.0420',
  );

  Future<WalletSummary> createWallet() async => WalletSummary(
        handle: _walletHandle,
        displayName: 'Architecture proof wallet',
        enabledChains: listChains(),
      );

  Future<WalletSummary> restoreWalletViaNativeSurface() async => WalletSummary(
        handle: _walletHandle,
        displayName: 'Restored proof wallet',
        enabledChains: listChains(),
      );

  WalletStatus walletStatus() => const WalletStatus(
        initialized: true,
        locked: false,
        enabled_chains: [_ethereum, _bitcoin],
        last_sync_at: null,
      );

  List<ChainId> listChains() => const [ChainId.ethereum, ChainId.bitcoin];

  List<ChainDescriptor> refreshAccounts() => const [_ethereum, _bitcoin];

  Future<PreparedHandle> prepareNativeTransfer({
    required ChainId chain,
    required String recipient,
    required BigInt amount,
  }) async {
    if (recipient.trim().isEmpty || amount <= BigInt.zero) {
      throw const BridgeException('Recipient and positive amount are required');
    }
    return PreparedHandle();
  }

  Future<String> authenticateSignAndBroadcast({
    required PreparedHandle prepared,
  }) async => '0x7d8f2a4c1b903e6a';

  TransactionStatus watchTransferStatus(String transactionId) =>
      TransactionStatus.Pending;

  String getReceiveAddress(ChainId chain) => switch (chain) {
        ChainId.ethereum => '0x1111111111111111111111111111111111111111',
        ChainId.bitcoin => 'tb1qexamplewalletaddress0000000000000000000',
      };

  Future<void> lockWallet() async {}

  Future<void> removeWallet() async {}
}
