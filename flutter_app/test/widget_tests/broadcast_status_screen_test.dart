import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:wallet_sample/src/screens/broadcast_status_screen.dart';
import 'package:wallet_sample/src/state/active_broadcast_controller.dart';

void main() {
  testWidgets('broadcast status renders every typed transfer stage', (tester) async {
    const labels = {
      TransferStage.preparing: 'Preparing',
      TransferStage.awaitingAuth: 'Awaiting authentication',
      TransferStage.signing: 'Signing',
      TransferStage.broadcasting: 'Broadcasting',
      TransferStage.submitted: 'Submitted',
      TransferStage.confirmed: 'Confirmed',
      TransferStage.failed: 'Failed',
    };

    for (final entry in labels.entries) {
      await tester.pumpWidget(
        MaterialApp(home: BroadcastStatusScreen(stage: entry.key)),
      );
      expect(find.text(entry.value), findsWidgets);
    }
  });
}
