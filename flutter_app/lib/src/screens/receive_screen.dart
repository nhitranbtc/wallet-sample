import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import '../bridge/bridge_facade_stub.dart';
import '../state/receive_controller.dart';
import '../theme/tokens.dart';
import '../widgets/network_badge.dart';
import '../widgets/qr_address_panel.dart';
import '../widgets/testnet_warning.dart';

class ReceiveScreen extends StatelessWidget {
  const ReceiveScreen({super.key, required this.controller});

  final ReceiveController controller;

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: controller,
      builder: (context, _) {
        final descriptor = _describe(controller.chain);
        return Scaffold(
          appBar: AppBar(title: const Text('Receive')),
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
              NetworkBadge(label: descriptor.network, isTestnet: true),
              const SizedBox(height: WalletSpacing.l),
              QrAddressPanel(
                address: controller.address,
                networkLabel: descriptor.label,
              ),
              const SizedBox(height: WalletSpacing.l),
              Row(
                children: [
                  Expanded(
                    child: OutlinedButton.icon(
                      icon: const Icon(Icons.copy),
                      label: const Text('Copy'),
                      onPressed: () =>
                          Clipboard.setData(ClipboardData(text: controller.address)),
                    ),
                  ),
                  const SizedBox(width: WalletSpacing.m),
                  Expanded(
                    child: OutlinedButton.icon(
                      icon: const Icon(Icons.share),
                      label: const Text('Share'),
                      onPressed: () {}, // Task 11 surfaces system share intent.
                    ),
                  ),
                ],
              ),
            ],
          ),
        );
      },
    );
  }

  _Descriptor _describe(ChainId chain) => switch (chain) {
        ChainId.ethereum => const _Descriptor(
            label: 'Ethereum Sepolia',
            network: 'Sepolia testnet',
          ),
        ChainId.bitcoin => const _Descriptor(
            label: 'Bitcoin Testnet',
            network: 'Bitcoin testnet',
          ),
      };
}

class _Descriptor {
  const _Descriptor({required this.label, required this.network});
  final String label;
  final String network;
}
