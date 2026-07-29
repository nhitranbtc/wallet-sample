import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:wallet_sample/src/bridge/bridge_facade_stub.dart';
import 'package:wallet_sample/src/screens/review_screen.dart';

void main() {
  testWidgets('review primary action authenticates before sending', (tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: ReviewScreen(
          chain: ChainId.bitcoin,
          recipient: 'tb1qexampledestination',
          amount: '0.001 BTC',
          fee: '0.00001 BTC',
        ),
      ),
    );

    expect(find.text('Authenticate and send'), findsOneWidget);
    expect(find.text('Simulate grant'), findsNothing);
    expect(find.widgetWithText(FilledButton, 'Send'), findsNothing);
  });
}
