import 'package:flutter/material.dart';

import '../state/settings_controller.dart';
import '../theme/tokens.dart';
import '../widgets/confirmation_dialog.dart';
import '../widgets/destructive_dialog.dart';

class SettingsScreen extends StatelessWidget {
  const SettingsScreen({
    super.key,
    required this.removeButtonFocusNode,
    required this.onLock,
  });

  final FocusNode removeButtonFocusNode;
  final Future<void> Function() onLock;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Settings')),
      body: ListView(
        padding: const EdgeInsets.all(WalletSpacing.l),
        children: [
          ListTile(
            leading: const Icon(Icons.lock_outline),
            title: const Text('Lock wallet'),
            subtitle: const Text('Requires biometric authentication'),
            onTap: () async {
              try {
                await onLock();
              } on StateError catch (error) {
                if (!context.mounted) return;
                ScaffoldMessenger.of(context).showSnackBar(
                  SnackBar(content: Text(error.message)),
                );
              }
            },
          ),
          ListTile(
            leading: const Icon(Icons.delete_outline),
            focusNode: removeButtonFocusNode,
            title: const Text('Remove wallet'),
            subtitle: const Text(
              'Permanently delete this wallet. Requires biometric authentication.',
            ),
            onTap: () async {
              await DestructiveDialog.show(
                context,
                restoreFocusTo: removeButtonFocusNode,
                title: 'Remove this wallet?',
                message:
                    'This permanently deletes the wallet on this device. '
                    'Testnet balances will be discarded.',
                confirmLabel: 'Remove wallet',
              );
            },
          ),
          const Divider(),
          ListTile(
            leading: const Icon(Icons.info_outline),
            title: const Text('Testnet build'),
            subtitle: const Text(
              'This build uses testnet chains only. Real funds are not supported.',
            ),
            onTap: () async {
              await showDialog<bool>(
                context: context,
                builder: (_) => const ConfirmationDialog(
                  title: 'Testnet build',
                  message: 'Real funds are not supported in this build.',
                  confirmLabel: 'Got it',
                ),
              );
            },
          ),
        ],
      ),
    );
  }
}
