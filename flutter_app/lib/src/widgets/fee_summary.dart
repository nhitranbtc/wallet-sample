import 'package:flutter/material.dart';

import '../theme/amount_text.dart';
import '../theme/tokens.dart';

class FeeSummary extends StatelessWidget {
  const FeeSummary({
    super.key,
    required this.fee,
    this.onRefresh,
    this.refreshing = false,
  });

  final String fee;
  final VoidCallback? onRefresh;
  final bool refreshing;

  @override
  Widget build(BuildContext context) {
    return DecoratedBox(
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.surfaceContainerLow,
        borderRadius: BorderRadius.circular(WalletRadius.l),
      ),
      child: Padding(
        padding: const EdgeInsets.all(WalletSpacing.l),
        child: Row(
          children: [
            const Expanded(child: Text('Estimated network fee')),
            if (refreshing)
              const SizedBox.square(
                dimension: WalletSpacing.xl,
                child: CircularProgressIndicator(strokeWidth: WalletElevation.m),
              )
            else ...[
              AmountText(fee, style: Theme.of(context).textTheme.bodyLarge),
              const SizedBox(width: WalletSpacing.s),
              IconButton(
                onPressed: onRefresh,
                tooltip: 'Refresh fee',
                icon: const Icon(Icons.refresh),
              ),
            ],
          ],
        ),
      ),
    );
  }
}
