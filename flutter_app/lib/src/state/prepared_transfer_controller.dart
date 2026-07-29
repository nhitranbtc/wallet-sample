import 'package:flutter/foundation.dart';

import '../bridge/bridge_facade_stub.dart';

enum PreparationState { idle, preparing, prepared, failed }

class PreparedTransferController extends ChangeNotifier {
  PreparedTransferController({required BridgeFacade bridge}) : _bridge = bridge;

  final BridgeFacade _bridge;
  PreparationState _state = PreparationState.idle;
  PreparedHandle? _prepared;
  String? _error;

  PreparationState get state => _state;
  PreparedHandle? get prepared => _prepared;
  String? get error => _error;

  Future<PreparedHandle> prepare({
    required ChainId chain,
    required String recipient,
    required BigInt amount,
  }) async {
    _state = PreparationState.preparing;
    _error = null;
    notifyListeners();
    try {
      final result = await _bridge.prepareNativeTransfer(
        chain: chain,
        recipient: recipient,
        amount: amount,
      );
      _prepared = result;
      _state = PreparationState.prepared;
      notifyListeners();
      return result;
    } on BridgeException catch (error) {
      _state = PreparationState.failed;
      _error = error.message;
      notifyListeners();
      rethrow;
    }
  }
}
