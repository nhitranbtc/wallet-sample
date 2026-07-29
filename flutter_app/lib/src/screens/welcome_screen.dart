import 'package:flutter/material.dart';

import '../state/onboarding_controller.dart';
import '../theme/tokens.dart';
import '../widgets/testnet_warning.dart';

class WelcomeScreen extends StatelessWidget {
  const WelcomeScreen({
    super.key,
    required this.controller,
    required this.onCreate,
    required this.onRestore,
  });

  final OnboardingController controller;
  final VoidCallback onCreate;
  final VoidCallback onRestore;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Ember Code Wallet')),
      body: Padding(
        padding: const EdgeInsets.all(WalletSpacing.xl),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            const TestnetWarning(),
            const SizedBox(height: WalletSpacing.xl),
            Text(
              'Hold testnet balances across chains while we ship the real bridge.',
              style: Theme.of(context).textTheme.titleMedium,
            ),
            const SizedBox(height: WalletSpacing.xxl),
            FilledButton(onPressed: onCreate, child: const Text('Create wallet')),
            const SizedBox(height: WalletSpacing.m),
            OutlinedButton(onPressed: onRestore, child: const Text('Restore wallet')),
          ],
        ),
      ),
    );
  }
}
