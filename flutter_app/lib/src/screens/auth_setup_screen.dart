import 'package:flutter/material.dart';

import '../state/onboarding_controller.dart';
import '../theme/tokens.dart';

class AuthSetupScreen extends StatelessWidget {
  const AuthSetupScreen({
    super.key,
    required this.controller,
    required this.onComplete,
  });

  final OnboardingController controller;
  final Future<void> Function() onComplete;

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: controller,
      builder: (context, _) {
        final busy = controller.state == OnboardingState.authSetup;
        return Scaffold(
          appBar: AppBar(title: const Text('Biometric setup')),
          body: Padding(
            padding: const EdgeInsets.all(WalletSpacing.xl),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                const Text(
                  'Confirm the device biometric prompt to finish wallet setup. '
                  'No live state leaves this device.',
                ),
                const SizedBox(height: WalletSpacing.xxl),
                FilledButton(
                  onPressed: busy
                      ? null
                      : () async {
                          try {
                            await onComplete();
                          } on StateError catch (_) {
                            if (!context.mounted) return;
                            ScaffoldMessenger.of(context).showSnackBar(
                              const SnackBar(
                                content: Text(
                                  'Biometric authentication is required to continue.',
                                ),
                              ),
                            );
                          }
                        },
                  child: busy
                      ? const SizedBox(
                          height: WalletSpacing.xl,
                          width: WalletSpacing.xl,
                          child: CircularProgressIndicator(strokeWidth: WalletElevation.m),
                        )
                      : const Text('Authorize and continue'),
                ),
              ],
            ),
          ),
        );
      },
    );
  }
}
