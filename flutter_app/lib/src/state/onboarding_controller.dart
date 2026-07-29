import 'package:flutter/foundation.dart';

import '../bridge/bridge_facade_stub.dart';
import 'biometric_gate.dart';

enum OnboardingState { welcome, recoveryNotice, authSetup, ready }

class OnboardingController extends ChangeNotifier {
  OnboardingController({
    required BridgeFacade bridge,
    required BiometricGate gate,
  })  : _bridge = bridge,
        _gate = gate;

  final BridgeFacade _bridge;
  final BiometricGate _gate;
  OnboardingState _state = OnboardingState.welcome;

  OnboardingState get state => _state;

  void startCreate() {
    _state = OnboardingState.recoveryNotice;
    notifyListeners();
  }

  void confirmRecoveryNotice() {
    _state = OnboardingState.authSetup;
    notifyListeners();
  }

  Future<void> completeAuthSetup() async {
    final granted = await _gate.promptForUnlock();
    if (!granted) {
      throw StateError('biometric authentication is required');
    }
    await _bridge.createWallet();
    _state = OnboardingState.ready;
    notifyListeners();
  }
}
