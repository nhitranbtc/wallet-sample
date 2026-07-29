import 'package:flutter/material.dart';

import '../theme/tokens.dart';

class TestnetWarning extends StatelessWidget {
  const TestnetWarning({super.key, this.network = 'Test networks'});

  final String network;

  @override
  Widget build(BuildContext context) {
    final dark = Theme.of(context).brightness == Brightness.dark;
    final background = dark
        ? WalletColors.warningSurfaceDark
        : WalletColors.warningSurfaceLight;
    final foreground = dark
        ? WalletColors.warningDark
        : WalletColors.warningLight;

    return Semantics(
      container: true,
      label: 'Warning: $network uses testnet funds only',
      child: Container(
        width: double.infinity,
        padding: const EdgeInsets.all(WalletSpacing.m),
        decoration: BoxDecoration(
          color: background,
          borderRadius: BorderRadius.circular(WalletRadius.m),
          border: Border.all(color: foreground),
        ),
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Icon(Icons.science_outlined, color: foreground),
            const SizedBox(width: WalletSpacing.s),
            Expanded(
              child: Text(
                '$network · Testnet funds have no real-world value.',
                style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                      color: foreground,
                    ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
