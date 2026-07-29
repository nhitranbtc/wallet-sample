import 'package:flutter/material.dart';

import 'tokens.dart';

ThemeData walletTheme(Brightness brightness) {
  final danger = brightness == Brightness.light
      ? WalletColors.dangerLight
      : WalletColors.dangerDark;
  final onDanger = brightness == Brightness.light
      ? WalletColors.onDangerLight
      : WalletColors.onDangerDark;
  final scheme = ColorScheme.fromSeed(
    seedColor: WalletColors.brandSeed,
    brightness: brightness,
  ).copyWith(error: danger, onError: onDanger);

  return ThemeData(
    brightness: brightness,
    colorScheme: scheme,
    useMaterial3: true,
    visualDensity: VisualDensity.adaptivePlatformDensity,
    scaffoldBackgroundColor: scheme.surface,
    inputDecorationTheme: InputDecorationTheme(
      filled: true,
      border: OutlineInputBorder(
        borderRadius: BorderRadius.circular(WalletRadius.l),
      ),
    ),
    filledButtonTheme: FilledButtonThemeData(
      style: FilledButton.styleFrom(
        minimumSize: const Size.fromHeight(WalletSpacing.xxxl),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(WalletRadius.l),
        ),
      ),
    ),
    dividerTheme: DividerThemeData(
      color: scheme.outlineVariant,
      space: WalletSpacing.xl,
    ),
  );
}
