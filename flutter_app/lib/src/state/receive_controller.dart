import 'package:flutter/foundation.dart';

import '../bridge/bridge_facade_stub.dart';

class ReceiveController extends ChangeNotifier {
  ReceiveController({required BridgeFacade bridge}) : _bridge = bridge {
    _address = _bridge.getReceiveAddress(_chain);
  }

  final BridgeFacade _bridge;
  ChainId _chain = ChainId.ethereum;
  late String _address;

  ChainId get chain => _chain;
  String get address => _address;

  void selectChain(ChainId chain) {
    _chain = chain;
    _address = _bridge.getReceiveAddress(chain);
    notifyListeners();
  }
}
