import 'package:flutter/material.dart';

import '../theme/tokens.dart';

class SkeletonBox extends StatelessWidget {
  const SkeletonBox({
    super.key,
    this.height = WalletSpacing.xxxl,
    this.width = double.infinity,
  });

  final double height;
  final double width;

  @override
  Widget build(BuildContext context) {
    return Semantics(
      label: 'Loading',
      child: Container(
        height: height,
        width: width,
        decoration: BoxDecoration(
          color: Theme.of(context).colorScheme.surfaceContainerHighest,
          borderRadius: BorderRadius.circular(WalletRadius.m),
        ),
      ),
    );
  }
}
