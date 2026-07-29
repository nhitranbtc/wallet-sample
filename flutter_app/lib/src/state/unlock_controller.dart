import 'package:flutter/foundation.dart';

import 'biometric_gate.dart';

class UnlockController extends ChangeNotifier {
  UnlockController({required BiometricGate gate, this.onUnlocked}) : _gate = gate;

  final BiometricGate _gate;
  final VoidCallback? onUnlocked;
  bool _unlocked = false;
  bool _busy = false;

  bool get unlocked => _unlocked;
  bool get busy => _busy;

  Future<void> unlock() async {
    _busy = true;
    notifyListeners();
    final authenticated = await _gate.promptForUnlock();
    _busy = false;
    if (!authenticated) {
      notifyListeners();
      throw StateError('unlock requires biometric authentication');
    }
    _unlocked = true;
    notifyListeners();
    onUnlocked?.call();
  }
}
