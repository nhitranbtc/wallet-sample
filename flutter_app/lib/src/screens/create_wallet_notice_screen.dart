import 'package:flutter/material.dart';

import '../state/onboarding_controller.dart';
import '../theme/tokens.dart';

class CreateWalletNoticeScreen extends StatelessWidget {
  const CreateWalletNoticeScreen({
    super.key,
    required this.controller,
    required this.onContinue,
    required this.onBack,
  });

  final OnboardingController controller;
  final VoidCallback onContinue;
  final VoidCallback onBack;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Recovery notice')),
      body: Padding(
        padding: const EdgeInsets.all(WalletSpacing.xl),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Text(
              'Hardware-isolated recovery',
              style: Theme.of(context).textTheme.headlineSmall,
            ),
            const SizedBox(height: WalletSpacing.m),
            const Text(
              'Recovery phrases are generated and stored in the platform secure '
              'surface (iOS Secure Enclave / Android Keystore attestation). '
              'They never travel through Dart or the bridge facade.',
            ),
            const SizedBox(height: WalletSpacing.xxl),
            FilledButton(onPressed: onContinue, child: const Text('Continue')),
            const SizedBox(height: WalletSpacing.s),
            TextButton(onPressed: onBack, child: const Text('Back')),
          ],
        ),
      ),
    );
  }
}
