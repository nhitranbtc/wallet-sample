import 'package:flutter/material.dart';

import '../bridge/bridge_facade_stub.dart';
import '../state/portfolio_controller.dart';
import '../theme/amount_text.dart';
import '../theme/tokens.dart';
import '../widgets/chain_card.dart';
import '../widgets/error_banner.dart';
import '../widgets/network_badge.dart';
import '../widgets/skeleton_box.dart';
import '../widgets/testnet_warning.dart';

class HomeScreen extends StatelessWidget {
  const HomeScreen({
    super.key,
    required this.state,
    this.chains = const [],
    this.errorMessage = 'We could not refresh balances. Try again.',
    this.onRefresh,
    this.onReceive,
    this.onSend,
    this.onLock,
  });

  final PortfolioViewState state;
  final List<ChainDescriptor> chains;
  final String errorMessage;
  final Future<void> Function()? onRefresh;
  final VoidCallback? onReceive;
  final VoidCallback? onSend;
  final VoidCallback? onLock;

  @override
  Widget build(BuildContext context) {
    final body = switch (state) {
      PortfolioViewState.loading => const _Loading(),
      PortfolioViewState.ready => _Ready(
          chains: chains,
          onReceive: onReceive,
          onSend: onSend,
          onLock: onLock,
        ),
      PortfolioViewState.stale => _Stale(chains: chains),
      PortfolioViewState.offline => _Offline(chains: chains),
      PortfolioViewState.error => _Error(
          message: errorMessage,
          onRetry: () => onRefresh?.call(),
        ),
    };

    return Scaffold(
      appBar: AppBar(
        title: const Text('Wallet'),
        actions: [
          if (state == PortfolioViewState.ready)
            IconButton(
              onPressed: onLock,
              icon: const Icon(Icons.lock_outline),
              tooltip: 'Lock wallet',
            ),
        ],
      ),
      body: RefreshIndicator(
        onRefresh: onRefresh ?? () async {},
        child: ListView(
          padding: const EdgeInsets.all(WalletSpacing.l),
          children: [
            const TestnetWarning(),
            const SizedBox(height: WalletSpacing.l),
            body,
          ],
        ),
      ),
    );
  }
}

class _Loading extends StatelessWidget {
  const _Loading();

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: const [
        LinearProgressIndicator(),
        SizedBox(height: WalletSpacing.l),
        SkeletonBox(),
        SizedBox(height: WalletSpacing.m),
        SkeletonBox(),
      ],
    );
  }
}

class _Ready extends StatelessWidget {
  const _Ready({
    required this.chains,
    this.onReceive,
    this.onSend,
    this.onLock,
  });

  final List<ChainDescriptor> chains;
  final VoidCallback? onReceive;
  final VoidCallback? onSend;
  final VoidCallback? onLock;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Text('Portfolio', style: Theme.of(context).textTheme.titleLarge),
        const SizedBox(height: WalletSpacing.m),
        for (final chain in chains) ...[
          ChainCard(chain: chain),
          const SizedBox(height: WalletSpacing.m),
        ],
        const SizedBox(height: WalletSpacing.l),
        Row(
          children: [
            Expanded(
              child: FilledButton.icon(
                onPressed: onReceive,
                icon: const Icon(Icons.arrow_downward),
                label: const Text('Receive'),
              ),
            ),
            const SizedBox(width: WalletSpacing.m),
            Expanded(
              child: FilledButton.icon(
                onPressed: onSend,
                icon: const Icon(Icons.arrow_upward),
                label: const Text('Send'),
              ),
            ),
          ],
        ),
      ],
    );
  }
}

class _Stale extends StatelessWidget {
  const _Stale({required this.chains});

  final List<ChainDescriptor> chains;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const ErrorBanner(message: 'Balances are out of date. Pull to refresh.'),
        const SizedBox(height: WalletSpacing.l),
        for (final chain in chains) ...[
          ChainCard(chain: chain),
          const SizedBox(height: WalletSpacing.m),
        ],
      ],
    );
  }
}

class _Offline extends StatelessWidget {
  const _Offline({required this.chains});

  final List<ChainDescriptor> chains;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const ErrorBanner(message: 'You are offline. Showing cached balances.'),
        const SizedBox(height: WalletSpacing.l),
        for (final chain in chains) ...[
          ChainCard(chain: chain),
          const SizedBox(height: WalletSpacing.m),
        ],
      ],
    );
  }
}

class _Error extends StatelessWidget {
  const _Error({required this.message, this.onRetry});

  final String message;
  final VoidCallback? onRetry;

  @override
  Widget build(BuildContext context) {
    return ErrorBanner(message: message, onRetry: onRetry);
  }
}
