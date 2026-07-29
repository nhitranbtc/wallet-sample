import 'package:flutter/material.dart';

import '../state/unlock_controller.dart';
import '../theme/tokens.dart';

class UnlockScreen extends StatelessWidget {
  const UnlockScreen({super.key, required this.controller});

  final UnlockController controller;

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: controller,
      builder: (context, _) {
        return Scaffold(
          body: Center(
            child: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: WalletBreakpoints.contentMax),
              child: Padding(
                padding: const EdgeInsets.all(WalletSpacing.xxl),
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    const Icon(Icons.lock_outline, size: WalletSpacing.xxxl * 2),
                    const SizedBox(height: WalletSpacing.xl),
                    Text(
                      'Wallet locked',
                      style: Theme.of(context).textTheme.headlineSmall,
                    ),
                    const SizedBox(height: WalletSpacing.m),
                    const Text('Authenticate to unlock the wallet.'),
                    const SizedBox(height: WalletSpacing.xxl),
                    FilledButton(
                      onPressed: controller.busy
                          ? null
                          : () async {
                              try {
                                await controller.unlock();
                              } on StateError catch (_) {
                                if (!context.mounted) return;
                                ScaffoldMessenger.of(context).showSnackBar(
                                  const SnackBar(
                                    content: Text(
                                      'Authentication is required to unlock.',
                                    ),
                                  ),
                                );
                              }
                            },
                      child: controller.busy
                          ? const SizedBox.square(
                              dimension: WalletSpacing.xl,
                              child: CircularProgressIndicator(
                                  strokeWidth: WalletElevation.m),
                            )
                          : const Text('Authenticate to unlock'),
                    ),
                  ],
                ),
              ),
            ),
          ),
        );
      },
    );
  }
}
