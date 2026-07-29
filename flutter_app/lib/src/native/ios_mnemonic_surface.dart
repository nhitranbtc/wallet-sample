import 'package:flutter/services.dart';

abstract final class IosMnemonicSurface {
  static const MethodChannel _channel = MethodChannel(
    'wallet_sample/ios_mnemonic_surface',
  );

  static Future<void> presentRestoreSurface() async {
    throw UnimplementedError(
      'Task 11 will bind the iOS native mnemonic surface through $_channel',
    );
  }
}
