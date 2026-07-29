import 'dart:mirrors';

import 'package:flutter_test/flutter_test.dart';
import 'package:wallet_sample/src/bridge/bridge_facade_stub.dart';

void main() {
  test('BridgeFacade exposes exactly the eleven frozen methods', () {
    final mirror = reflectClass(BridgeFacade);
    final methods = mirror.declarations.values
        .whereType<MethodMirror>()
        .where((method) => !method.isConstructor && !method.isGetter && !method.isSetter)
        .map((method) => MirrorSystem.getName(method.simpleName))
        .toSet();

    const expected = {
      'createWallet',
      'restoreWalletViaNativeSurface',
      'walletStatus',
      'listChains',
      'refreshAccounts',
      'prepareNativeTransfer',
      'authenticateSignAndBroadcast',
      'watchTransferStatus',
      'getReceiveAddress',
      'lockWallet',
      'removeWallet',
    };

    expect(methods, expected);
    expect(methods, hasLength(11));
  });
}
