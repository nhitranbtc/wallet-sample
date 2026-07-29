import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:wallet_sample/src/theme/theme_data.dart';

void main() {
  for (final brightness in Brightness.values) {
    testWidgets('${brightness.name} wallet theme renders', (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          theme: walletTheme(brightness),
          home: const Scaffold(body: Text('Wallet')),
        ),
      );

      final context = tester.element(find.text('Wallet'));
      expect(Theme.of(context).brightness, brightness);
      expect(Theme.of(context).useMaterial3, isTrue);
    });
  }
}
