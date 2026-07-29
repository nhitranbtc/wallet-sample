import 'package:flutter/material.dart';

import '../theme/tokens.dart';

class RestoreWalletScreen extends StatelessWidget {
  const RestoreWalletScreen({super.key, required this.onBeginSurface});

  final VoidCallback onBeginSurface;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Restore wallet')),
      body: Padding(
        padding: const EdgeInsets.all(WalletSpacing.xl),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            const Text(
              'Restore happens through the platform secure surface. '
              'Dart never receives mnemonic words.',
            ),
            const SizedBox(height: WalletSpacing.xxl),
            FilledButton.icon(
              onPressed: onBeginSurface,
              icon: const Icon(Icons.lock_outline),
              label: const Text('Open secure recovery'),
            ),
          ],
        ),
      ),
    );
  }
}
