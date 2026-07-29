import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import '../state/active_broadcast_controller.dart';
import '../theme/amount_text.dart';
import '../theme/tokens.dart';
import '../widgets/error_banner.dart';
import '../widgets/status_timeline.dart';

class BroadcastStatusScreen extends StatelessWidget {
  const BroadcastStatusScreen({
    super.key,
    required this.stage,
    this.transactionHash,
    this.errorMessage,
    this.onRefresh,
  });

  final TransferStage stage;
  final String? transactionHash;
  final String? errorMessage;
  final VoidCallback? onRefresh;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Transfer status')),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: WalletBreakpoints.contentMax),
          child: ListView(
            padding: const EdgeInsets.all(WalletSpacing.l),
            children: [
              if (errorMessage != null) ...[
                ErrorBanner(message: errorMessage!),
                const SizedBox(height: WalletSpacing.l),
              ],
              LiveRegion(
                label: 'Transfer status: ${stage.label}',
                child: StatusTimeline(currentStage: stage),
              ),
              if (transactionHash != null) ...[
                const SizedBox(height: WalletSpacing.xl),
                Card(
                  child: Padding(
                    padding: const EdgeInsets.all(WalletSpacing.l),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          'Transaction hash',
                          style: Theme.of(context).textTheme.titleSmall,
                        ),
                        const SizedBox(height: WalletSpacing.s),
                        SelectableText(transactionHash!),
                        const SizedBox(height: WalletSpacing.s),
                        AmountText(transactionHash!),
                        const SizedBox(height: WalletSpacing.s),
                        Semantics(
                          link: true,
                          label: 'Open explorer for transaction $transactionHash',
                          child: Text(
                            'View on explorer',
                            style: Theme.of(context)
                                .textTheme
                                .bodyMedium
                                ?.copyWith(
                                  color: Theme.of(context).colorScheme.primary,
                                ),
                          ),
                        ),
                        const SizedBox(height: WalletSpacing.s),
                        TextButton.icon(
                          onPressed: () async {
                            await Clipboard.setData(
                              ClipboardData(text: transactionHash!),
                            );
                            if (!context.mounted) return;
                            ScaffoldMessenger.of(context).showSnackBar(
                              const SnackBar(content: Text('Hash copied')),
                            );
                          },
                          icon: const Icon(Icons.copy),
                          label: const Text('Copy hash'),
                        ),
                      ],
                    ),
                  ),
                ),
              ],
              if (onRefresh != null) ...[
                const SizedBox(height: WalletSpacing.l),
                FilledButton(
                  onPressed: onRefresh,
                  child: const Text('Refresh status'),
                ),
              ],
            ],
          ),
        ),
      ),
    );
  }
}
