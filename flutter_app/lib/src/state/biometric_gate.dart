import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import 'package:local_auth/local_auth.dart';

class BiometricGate {
  BiometricGate({LocalAuthentication? authentication})
      : _auth = authentication ?? LocalAuthentication();

  final LocalAuthentication _auth;

  Future<bool> promptForSigning() => _authenticate(
        'Authenticate to sign this transaction',
      );

  Future<bool> promptForUnlock() => _authenticate('Unlock wallet');

  Future<bool> promptForDestructive(String label) =>
      _authenticate('Confirm $label');

  Future<bool> isAvailable() async {
    try {
      return await _auth.canCheckBiometrics || await _auth.isDeviceSupported();
    } on MissingPluginException {
      return false;
    } on PlatformException {
      return false;
    } catch (error) {
      debugPrint('Biometric availability check failed: $error');
      return false;
    }
  }

  Future<bool> _authenticate(String reason) async {
    try {
      return await _auth.authenticate(
        localizedReason: reason,
        options: const AuthenticationOptions(
          biometricOnly: false,
          stickyAuth: true,
        ),
      );
    } on MissingPluginException {
      return false;
    } on PlatformException {
      return false;
    } catch (error) {
      debugPrint('Biometric prompt failed: $error');
      return false;
    }
  }
}
