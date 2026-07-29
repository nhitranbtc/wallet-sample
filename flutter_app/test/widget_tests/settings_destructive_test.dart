import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:wallet_sample/src/screens/settings_screen.dart';
import 'package:wallet_sample/src/theme/theme_data.dart';
import 'package:wallet_sample/src/theme/tokens.dart';

void main() {
  testWidgets('destructive dialog uses danger token and cancel restores focus', (tester) async {
    final focusNode = FocusNode();
    addTearDown(focusNode.dispose);

    await tester.pumpWidget(
      MaterialApp(
        theme: walletTheme(Brightness.light),
        home: SettingsScreen(removeButtonFocusNode: focusNode),
      ),
    );

    await tester.tap(find.text('Remove wallet'));
    await tester.pumpAndSettle();

    final dialog = tester.widget<AlertDialog>(find.byType(AlertDialog));
    expect(dialog.backgroundColor, WalletColors.dangerLight);

    await tester.tap(find.text('Cancel'));
    await tester.pumpAndSettle();
    expect(focusNode.hasFocus, isTrue);
  });
}
