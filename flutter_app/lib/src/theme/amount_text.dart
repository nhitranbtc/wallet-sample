import 'dart:ui' show FontFeature;

import 'package:flutter/material.dart';

class AmountText extends StatelessWidget {
  const AmountText(this.value, {super.key, this.style});

  final String value;
  final TextStyle? style;

  @override
  Widget build(BuildContext context) {
    return Text(
      value,
      style: (style ?? Theme.of(context).textTheme.titleLarge)?.copyWith(
        fontFeatures: const [FontFeature.tabularFigures()],
      ),
    );
  }
}

class MonoText extends StatelessWidget {
  const MonoText(this.value, {super.key, this.semanticLabel});

  final String value;
  final String? semanticLabel;

  @override
  Widget build(BuildContext context) {
    return Semantics(
      label: semanticLabel,
      child: SelectableText(
        value,
        style: Theme.of(context).textTheme.bodyMedium?.copyWith(
          fontFamily: 'monospace',
          fontFamilyFallback: const ['Courier', 'RobotoMono'],
          fontFeatures: const [FontFeature.tabularFigures()],
        ),
      ),
    );
  }
}
