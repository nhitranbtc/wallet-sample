import 'package:flutter_test/flutter_test.dart';
import 'package:wallet_sample/src/bridge/bridge_facade_stub.dart';
import 'package:wallet_sample/src/state/biometric_gate.dart';
import 'package:wallet_sample/src/state/onboarding_controller.dart';

class _Gate extends BiometricGate {
  _Gate(this.granted);
  final bool granted;

  @override
  Future<bool> promptForUnlock() async => granted;
}

class _Bridge extends BridgeFacade {
  int createCalls = 0;

  @override
  Future<WalletSummary> createWallet() async {
    createCalls += 1;
    return super.createWallet();
  }
}

void main() {
  test('onboarding reaches ready only after successful authentication', () async {
    final bridge = _Bridge();
    final controller = OnboardingController(bridge: bridge, gate: _Gate(true));

    expect(controller.state, OnboardingState.welcome);
    controller.startCreate();
    expect(controller.state, OnboardingState.recoveryNotice);
    controller.confirmRecoveryNotice();
    expect(controller.state, OnboardingState.authSetup);
    await controller.completeAuthSetup();

    expect(controller.state, OnboardingState.ready);
    expect(bridge.createCalls, 1);
  });

  test('onboarding remains at auth setup when authentication is denied', () async {
    final bridge = _Bridge();
    final controller = OnboardingController(bridge: bridge, gate: _Gate(false));
    controller.startCreate();
    controller.confirmRecoveryNotice();

    await expectLater(controller.completeAuthSetup(), throwsStateError);
    expect(controller.state, OnboardingState.authSetup);
    expect(bridge.createCalls, 0);
  });
}
