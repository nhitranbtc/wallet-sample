import 'package:flutter/material.dart';

import '../theme/tokens.dart';

class DestructiveDialog extends StatelessWidget {
  const DestructiveDialog({
    super.key,
    required this.title,
    required this.message,
    required this.confirmLabel,
  });

  final String title;
  final String message;
  final String confirmLabel;

  static Future<bool> show(
    BuildContext context, {
    required FocusNode restoreFocusTo,
    required String title,
    required String message,
    required String confirmLabel,
  }) async {
    final confirmed = await showDialog<bool>(
          context: context,
          builder: (_) => DestructiveDialog(
            title: title,
            message: message,
            confirmLabel: confirmLabel,
          ),
        ) ??
        false;
    if (!confirmed) restoreFocusTo.requestFocus();
    return confirmed;
  }

  @override
  Widget build(BuildContext context) {
    final scheme = Theme.of(context).colorScheme;
    return Semantics(
      container: true,
      label: 'Danger: $title',
      child: AlertDialog(
        backgroundColor: scheme.error,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(WalletRadius.l),
        ),
        title: Text(title, style: TextStyle(color: scheme.onError)),
        content: Padding(
          padding: const EdgeInsets.only(top: WalletSpacing.xs),
          child: Text(message, style: TextStyle(color: scheme.onError)),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(false),
            style: TextButton.styleFrom(foregroundColor: scheme.onError),
            child: const Text('Cancel'),
          ),
          FilledButton(
            onPressed: () => Navigator.of(context).pop(true),
            style: FilledButton.styleFrom(
              backgroundColor: scheme.onError,
              foregroundColor: scheme.error,
            ),
            child: Text(confirmLabel),
          ),
        ],
      ),
    );
  }
}
