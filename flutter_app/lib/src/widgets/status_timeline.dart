import 'package:flutter/material.dart';

import '../state/active_broadcast_controller.dart';
import '../theme/tokens.dart';

class LiveRegion extends StatelessWidget {
  const LiveRegion({super.key, required this.child, this.label});

  final Widget child;
  final String? label;

  @override
  Widget build(BuildContext context) {
    return Semantics(liveRegion: true, label: label, child: child);
  }
}

class StatusTimeline extends StatelessWidget {
  const StatusTimeline({super.key, required this.currentStage});

  final TransferStage currentStage;

  @override
  Widget build(BuildContext context) {
    return LiveRegion(
      label: 'Transfer status: ${currentStage.label}',
      child: Column(
        children: [
          for (final stage in TransferStage.values)
            _StageRow(stage: stage, currentStage: currentStage),
        ],
      ),
    );
  }
}

class _StageRow extends StatelessWidget {
  const _StageRow({required this.stage, required this.currentStage});

  final TransferStage stage;
  final TransferStage currentStage;

  @override
  Widget build(BuildContext context) {
    final currentIndex = TransferStage.values.indexOf(currentStage);
    final stageIndex = TransferStage.values.indexOf(stage);
    final reached = stageIndex <= currentIndex && currentStage != TransferStage.failed;
    final isCurrent = stage == currentStage;
    final color = isCurrent
        ? Theme.of(context).colorScheme.primary
        : reached
            ? WalletColors.successLight
            : Theme.of(context).colorScheme.outline;

    return Padding(
      padding: const EdgeInsets.symmetric(vertical: WalletSpacing.s),
      child: Row(
        children: [
          Icon(
            reached ? Icons.check_circle : Icons.radio_button_unchecked,
            color: color,
            size: WalletSpacing.xl,
          ),
          const SizedBox(width: WalletSpacing.m),
          Expanded(
            child: Text(
              stage.label,
              style: isCurrent
                  ? Theme.of(context).textTheme.titleSmall
                  : Theme.of(context).textTheme.bodyMedium,
            ),
          ),
        ],
      ),
    );
  }
}
