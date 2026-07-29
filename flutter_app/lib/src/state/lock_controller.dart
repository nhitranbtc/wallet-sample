import 'package:flutter/foundation.dart';

import '../bridge/bridge_facade_stub.dart';
import 'biometric_gate.dart';

class LockController extends ChangeNotifier {
  LockController({required BridgeFacade bridge, required BiometricGate gate})
      : _bridge = bridge,
        _gate = gate;

  final BridgeFacade _bridge;
  final BiometricGate _gate;
  bool _locked = false;

  bool get locked => _locked;

  Future<void> lock() async {
    final authenticated =
        await _gate.promptForDestructive('lock wallet');
    if (!authenticated) {
      throw StateError('lock requires biometric authentication');
    }
    await _bridge.lockWallet();
    _locked = true;
    notifyListeners();
  }
}
