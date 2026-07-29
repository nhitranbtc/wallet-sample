import 'package:flutter/services.dart';

abstract final class AndroidMnemonicSurface {
  static const MethodChannel _channel = MethodChannel(
    'wallet_sample/android_mnemonic_surface',
  );

  static Future<void> presentRestoreSurface() async {
    throw UnimplementedError(
      'Task 11 will bind the Android native mnemonic surface through $_channel',
    );
  }
}
