import 'package:flutter/material.dart';

import '../bridge/bridge_facade_stub.dart';
import '../theme/amount_text.dart';
import '../theme/tokens.dart';
import 'network_badge.dart';

class ChainCard extends StatelessWidget {
  const ChainCard({super.key, required this.chain, this.onTap});

  final ChainDescriptor chain;
  final VoidCallback? onTap;

  @override
  Widget build(BuildContext context) {
    return Card(
      elevation: WalletElevation.s,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(WalletRadius.l),
      ),
      child: InkWell(
        borderRadius: BorderRadius.circular(WalletRadius.l),
        onTap: onTap,
        child: Padding(
          padding: const EdgeInsets.all(WalletSpacing.l),
          child: Row(
            children: [
              CircleAvatar(child: Text(chain.symbol.characters.first)),
              const SizedBox(width: WalletSpacing.m),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(chain.name, style: Theme.of(context).textTheme.titleMedium),
                    const SizedBox(height: WalletSpacing.xs),
                    NetworkBadge(
                      label: chain.network,
                      isTestnet: chain.isTestnet,
                    ),
                  ],
                ),
              ),
              AmountText('${chain.balance} ${chain.symbol}'),
            ],
          ),
        ),
      ),
    );
  }
}
