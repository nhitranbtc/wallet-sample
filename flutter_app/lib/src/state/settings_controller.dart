import 'package:flutter/foundation.dart';

import '../bridge/bridge_facade_stub.dart';
import 'biometric_gate.dart';

class SettingsController extends ChangeNotifier {
  SettingsController({required BridgeFacade bridge, required BiometricGate gate})
      : _bridge = bridge,
        _gate = gate;

  final BridgeFacade _bridge;
  final BiometricGate _gate;
  bool _removed = false;

  bool get removed => _removed;

  Future<void> removeWallet() async {
    final authenticated =
        await _gate.promptForDestructive('remove wallet');
    if (!authenticated) {
      throw StateError('wallet removal requires biometric authentication');
    }
    await _bridge.removeWallet();
    _removed = true;
    notifyListeners();
  }
}
