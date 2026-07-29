import 'package:flutter/material.dart';

import '../theme/tokens.dart';

class NetworkBadge extends StatelessWidget {
  const NetworkBadge({
    super.key,
    required this.label,
    required this.isTestnet,
  });

  final String label;
  final bool isTestnet;

  @override
  Widget build(BuildContext context) {
    final brightness = Theme.of(context).brightness;
    final warning = brightness == Brightness.light
        ? WalletColors.warningLight
        : WalletColors.warningDark;

    return Semantics(
      label: isTestnet ? '$label, test network' : label,
      child: Container(
        padding: const EdgeInsets.symmetric(
          horizontal: WalletSpacing.s,
          vertical: WalletSpacing.xs,
        ),
        decoration: BoxDecoration(
          border: Border.all(
            color: isTestnet ? warning : Theme.of(context).colorScheme.outline,
          ),
          borderRadius: BorderRadius.circular(WalletRadius.pill),
        ),
        child: Text(
          label,
          style: Theme.of(context).textTheme.labelSmall?.copyWith(
                color: isTestnet ? warning : null,
              ),
        ),
      ),
    );
  }
}
