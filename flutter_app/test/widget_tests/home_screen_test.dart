import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:wallet_sample/src/screens/home_screen.dart';
import 'package:wallet_sample/src/state/portfolio_controller.dart';

void main() {
  for (final state in PortfolioViewState.values) {
    testWidgets('home renders the ${state.name} state', (tester) async {
      await tester.pumpWidget(MaterialApp(home: HomeScreen(state: state)));

      switch (state) {
        case PortfolioViewState.loading:
          expect(find.byType(LinearProgressIndicator), findsOneWidget);
        case PortfolioViewState.ready:
          expect(find.text('Portfolio'), findsOneWidget);
        case PortfolioViewState.stale:
          expect(find.textContaining('out of date'), findsOneWidget);
        case PortfolioViewState.offline:
          expect(find.textContaining('offline'), findsOneWidget);
        case PortfolioViewState.error:
          expect(find.textContaining('could not refresh'), findsOneWidget);
      }
    });
  }
}
