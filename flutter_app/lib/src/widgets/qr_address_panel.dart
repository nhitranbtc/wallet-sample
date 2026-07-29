import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:qr_flutter/qr_flutter.dart';

import '../theme/amount_text.dart';
import '../theme/tokens.dart';

class QrAddressPanel extends StatelessWidget {
  const QrAddressPanel({
    super.key,
    required this.address,
    required this.networkLabel,
  });

  final String address;
  final String networkLabel;

  Future<void> _copyAddress(BuildContext context) async {
    await Clipboard.setData(ClipboardData(text: address));
    if (!context.mounted) return;
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('Address copied')),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Semantics(
      container: true,
      label: 'QR code and receive address for $networkLabel. $address',
      child: Container(
        padding: const EdgeInsets.all(WalletSpacing.xl),
        decoration: BoxDecoration(
          color: Theme.of(context).colorScheme.surfaceContainerLowest,
          borderRadius: BorderRadius.circular(WalletRadius.xl),
          border: Border.all(color: Theme.of(context).colorScheme.outlineVariant),
        ),
        child: Column(
          children: [
            ExcludeSemantics(
              child: QrImageView(
                data: address,
                size: WalletSpacing.xxxl * 4,
              ),
            ),
            const SizedBox(height: WalletSpacing.l),
            MonoText(address, semanticLabel: 'Canonical receive address'),
            const SizedBox(height: WalletSpacing.m),
            OutlinedButton.icon(
              onPressed: () => _copyAddress(context),
              icon: const Icon(Icons.copy),
              label: const Text('Copy address'),
            ),
          ],
        ),
      ),
    );
  }
}
