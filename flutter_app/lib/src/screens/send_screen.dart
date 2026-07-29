import 'package:flutter/material.dart';

import '../bridge/bridge_facade_stub.dart';
import '../state/send_draft_controller.dart';
import '../theme/amount_text.dart';
import '../theme/tokens.dart';
import '../widgets/address_input.dart';
import '../widgets/amount_input.dart';
import '../widgets/fee_summary.dart';
import '../widgets/testnet_warning.dart';

class SendScreen extends StatefulWidget {
  const SendScreen({
    super.key,
    required this.controller,
    required this.onReview,
  });

  final SendDraftController controller;
  final ValueChanged<bool> onReview;

  @override
  State<SendScreen> createState() => _SendScreenState();
}

class _SendScreenState extends State<SendScreen> {
  final TextEditingController _recipient = TextEditingController();
  final TextEditingController _amount = TextEditingController();

  @override
  void initState() {
    super.initState();
    _recipient.text = widget.controller.recipient;
    _amount.text = widget.controller.amount;
    widget.controller.addListener(_syncFromDraft);
  }

  @override
  void didUpdateWidget(SendScreen oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.controller != widget.controller) {
      oldWidget.controller.removeListener(_syncFromDraft);
      widget.controller.addListener(_syncFromDraft);
      _syncFromDraft();
    }
  }

  @override
  void dispose() {
    widget.controller.removeListener(_syncFromDraft);
    _recipient.dispose();
    _amount.dispose();
    super.dispose();
  }

  /// Mirrors draft-side mutations (e.g. `useMax`) back into the text fields
  /// without echoing user keystrokes, which already flow draft-ward.
  void _syncFromDraft() {
    _applyText(_recipient, widget.controller.recipient);
    _applyText(_amount, widget.controller.amount);
  }

  void _applyText(TextEditingController field, String value) {
    if (field.text == value) return;
    field.value = TextEditingValue(
      text: value,
      selection: TextSelection.collapsed(offset: value.length),
    );
  }

  @override
  Widget build(BuildContext context) {
    final controller = widget.controller;
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
                controller: _recipient,
                onChanged: controller.updateRecipient,
              ),
              AmountInput(
                controller: _amount,
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
                onPressed: enabled ? () => widget.onReview(true) : null,
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
