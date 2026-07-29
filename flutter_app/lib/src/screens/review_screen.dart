import 'package:flutter/material.dart';

import '../bridge/bridge_facade_stub.dart';
import '../theme/amount_text.dart';
import '../theme/tokens.dart';
import '../widgets/network_badge.dart';
import '../widgets/review_panel.dart';
import '../widgets/testnet_warning.dart';

class ReviewScreen extends StatefulWidget {
  const ReviewScreen({
    super.key,
    required this.chain,
    required this.recipient,
    required this.amount,
    required this.fee,
  });

  final ChainId chain;
  final String recipient;
  final String amount;
  final String fee;

  @override
  State<ReviewScreen> createState() => _ReviewScreenState();
}

class _ReviewScreenState extends State<ReviewScreen> {
  bool _chainDetailsExpanded = false;

  @override
  Widget build(BuildContext context) {
    final descriptor = _describe(widget.chain);
    return Scaffold(
      appBar: AppBar(title: const Text('Review transfer')),
      body: ListView(
        padding: const EdgeInsets.all(WalletSpacing.l),
        children: [
          const TestnetWarning(),
          const SizedBox(height: WalletSpacing.l),
          ReviewPanel(children: [
            Row(
              children: [
                Expanded(child: Text(widget.amount)),
                Text(descriptor.symbol,
                    style: Theme.of(context).textTheme.titleMedium),
              ],
            ),
            Row(
              children: [
                const Expanded(child: Text('Network fee')),
                AmountText(widget.fee),
              ],
            ),
            Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Expanded(child: Text(descriptor.name)),
                    NetworkBadge(
                      label: descriptor.network,
                      isTestnet: descriptor.isTestnet,
                    ),
                  ],
                ),
                TextButton.icon(
                  onPressed: () => setState(
                    () => _chainDetailsExpanded = !_chainDetailsExpanded,
                  ),
                  icon: Icon(
                    _chainDetailsExpanded
                        ? Icons.expand_less
                        : Icons.expand_more,
                  ),
                  label: Text(
                    _chainDetailsExpanded
                        ? 'Hide chain details'
                        : 'Show chain details',
                  ),
                ),
                if (_chainDetailsExpanded)
                  Padding(
                    padding:
                        const EdgeInsets.only(top: WalletSpacing.s),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(descriptor.summary),
                        const SizedBox(height: WalletSpacing.xs),
                        Text('Network: ${descriptor.network}'),
                        Text('Asset: ${descriptor.symbol}'),
                      ],
                    ),
                  ),
              ],
            ),
            SelectableText('To: ${widget.recipient}'),
          ]),
          const SizedBox(height: WalletSpacing.l),
          FilledButton(
            onPressed: () => Navigator.of(context).pushNamed('/auth'),
            child: const Text('Authenticate and send'),
          ),
          const SizedBox(height: WalletSpacing.s),
          TextButton(
            onPressed: () => Navigator.of(context).maybePop(),
            child: const Text('Back to draft'),
          ),
        ],
      ),
    );
  }

  _ReviewDescriptor _describe(ChainId chain) => switch (chain) {
        ChainId.ethereum => const _ReviewDescriptor(
            name: 'Ethereum',
            symbol: 'ETH',
            network: 'Sepolia testnet',
            summary:
                'Native ETH transfers on the Sepolia testnet. Test-only funds.',
            isTestnet: true,
          ),
        ChainId.bitcoin => const _ReviewDescriptor(
            name: 'Bitcoin',
            symbol: 'BTC',
            network: 'Bitcoin testnet',
            summary:
                'Native BTC transfers on the Bitcoin testnet. Test-only funds.',
            isTestnet: true,
          ),
      };
}

class _ReviewDescriptor {
  const _ReviewDescriptor({
    required this.name,
    required this.symbol,
    required this.network,
    required this.summary,
    required this.isTestnet,
  });

  final String name;
  final String symbol;
  final String network;
  final String summary;
  final bool isTestnet;
}
