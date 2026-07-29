import 'package:flutter/material.dart';

abstract final class WalletColors {
  static const Color brandSeed = Color(0xFF5B4AE8);
  static const Color dangerLight = Color(0xFFB3261E);
  static const Color dangerDark = Color(0xFFF2B8B5);
  static const Color onDangerLight = Color(0xFFFFFFFF);
  static const Color onDangerDark = Color(0xFF601410);
  static const Color warningLight = Color(0xFF9A4D00);
  static const Color warningDark = Color(0xFFFFB95C);
  static const Color onWarningLight = Color(0xFFFFFFFF);
  static const Color onWarningDark = Color(0xFF502400);
  static const Color warningSurfaceLight = Color(0xFFFFE9CC);
  static const Color warningSurfaceDark = Color(0xFF442B0C);
  static const Color successLight = Color(0xFF146C43);
  static const Color successDark = Color(0xFF76D5A5);
}

abstract final class WalletSpacing {
  static const double none = 0;
  static const double xs = 4;
  static const double s = 8;
  static const double m = 12;
  static const double l = 16;
  static const double xl = 24;
  static const double xxl = 32;
  static const double xxxl = 48;
}

abstract final class WalletRadius {
  static const double s = 4;
  static const double m = 8;
  static const double l = 12;
  static const double xl = 16;
  static const double pill = 999;
}

abstract final class WalletElevation {
  static const double none = 0;
  static const double s = 1;
  static const double m = 3;
  static const double l = 8;
}

abstract final class WalletMotion {
  static const Duration short = Duration(milliseconds: 120);
  static const Duration medium = Duration(milliseconds: 220);
  static const Duration long = Duration(milliseconds: 360);
}

abstract final class WalletBreakpoints {
  static const double navigationRail = 720;
  static const double contentMax = 880;
}
