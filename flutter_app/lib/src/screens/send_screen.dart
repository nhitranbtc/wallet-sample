import 'package:flutter/material.dart';

import '../bridge/bridge_facade_stub.dart';
import '../state/send_draft_controller.dart';
import '../theme/amount_text.dart';
import '../theme/tokens.dart';
import '../widgets/address_input.dart';
import '../widgets/amount_input.dart';
import '../widgets/fee_summary.dart';
import '../widgets/testnet_warning.dart';

class SendScreen extends StatelessWidget {
  const SendScreen({
    super.key,
    required this.controller,
    required this.onReview,
  });

  final SendDraftController controller;
  final ValueChanged<bool> onReview;

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: controller,
      builder: (context, _) {
        final enabled = controller.validate(controller.amount) == null &&
            controller.recipient.trim().isNotEmpty;
        return Scaffold(
          appBar: AppBar(title: const Text('Send')),
          body: ListView(
            padding: const EdgeInsets.all(WalletSpacing.l),
            children: [
              const TestnetWarning(),
              const SizedBox(height: WalletSpacing.l),
              SegmentedButton<ChainId>(
                segments: const [
                  ButtonSegment(
                    value: ChainId.ethereum,
                    label: Text('Ethereum'),
                  ),
                  ButtonSegment(
                    value: ChainId.bitcoin,
                    label: Text('Bitcoin'),
                  ),
                ],
                selected: {controller.chain},
                onSelectionChanged: (selection) =>
                    controller.selectChain(selection.first),
              ),
              const SizedBox(height: WalletSpacing.m),
              BalanceRow(
                chain: controller.chain,
                balance: controller.availableBalance.toString(),
              ),
              AddressInput(
                controller: TextEditingController(text: controller.recipient),
                onChanged: controller.updateRecipient,
              ),
              AmountInput(
                controller: TextEditingController(text: controller.amount),
                validator: controller.validate,
                onChanged: controller.updateAmount,
                onMax: controller.useMax,
              ),
              FeeSummary(
                fee: controller.fee,
                refreshing: controller.fee == 'Fee refresh required',
                onRefresh: controller.refreshFee,
              ),
              const SizedBox(height: WalletSpacing.l),
              FilledButton(
                onPressed: enabled ? () => onReview(true) : null,
                child: const Text('Review transfer'),
              ),
            ],
          ),
        );
      },
    );
  }
}

class BalanceRow extends StatelessWidget {
  const BalanceRow({super.key, required this.chain, required this.balance});

  final ChainId chain;
  final String balance;

  @override
  Widget build(BuildContext context) {
    final unit = chain == ChainId.ethereum ? 'ETH' : 'BTC';
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: WalletSpacing.s),
      child: Row(
        children: [
          const Expanded(child: Text('Available balance')),
          AmountText('$balance $unit'),
        ],
      ),
    );
  }
}
