import 'package:flutter/material.dart';

import 'src/bridge/bridge_facade_stub.dart';
import 'src/screens/auth_setup_screen.dart';
import 'src/screens/authentication_screen.dart';
import 'src/screens/broadcast_status_screen.dart';
import 'src/screens/create_wallet_notice_screen.dart';
import 'src/screens/home_screen.dart';
import 'src/screens/receive_screen.dart';
import 'src/screens/review_screen.dart';
import 'src/screens/send_screen.dart';
import 'src/screens/settings_screen.dart';
import 'src/screens/welcome_screen.dart';
import 'src/shell/wallet_shell.dart';
import 'src/state/app_state.dart';
import 'src/state/biometric_gate.dart';
import 'src/state/active_broadcast_controller.dart';
import 'src/state/onboarding_controller.dart';
import 'src/state/receive_controller.dart';
import 'src/state/send_draft_controller.dart';
import 'src/theme/theme_data.dart';

void main() {
  runApp(const WalletApp());
}

class WalletApp extends StatefulWidget {
  const WalletApp({super.key});

  @override
  State<WalletApp> createState() => _WalletAppState();
}

class _WalletAppState extends State<WalletApp> {
  late final BridgeFacade _bridge = BridgeFacade();
  late final BiometricGate _gate = BiometricGate();
  late final AppState _appState = AppState();
  late final OnboardingController _onboarding =
      OnboardingController(bridge: _bridge, gate: _gate);
  late final ReceiveController _receiveController =
      ReceiveController(bridge: _bridge);
  late final SendDraftController _draftController = SendDraftController();
  late final ActiveBroadcastController _broadcast = ActiveBroadcastController(
    bridge: _bridge,
    gate: _gate,
  );

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Ember Code Wallet',
      theme: walletTheme(Brightness.light),
      darkTheme: walletTheme(Brightness.dark),
      themeMode: ThemeMode.system,
      routes: {
        '/auth': (_) => const AuthenticationScreen(),
        '/broadcast': (_) => const BroadcastStatusScreen(
              stage: ActiveBroadcastController.placeholderStage,
            ),
      },
      home: _buildLanding(),
    );
  }

  Widget _buildLanding() {
    return ListenableBuilder(
      listenable: Listenable.merge([_appState, _onboarding]),
      builder: (context, _) => switch (_onboarding.state) {
        OnboardingState.welcome => WelcomeScreen(
            controller: _onboarding,
            onCreate: _onboarding.startCreate,
            onRestore: () {},
          ),
        OnboardingState.recoveryNotice => CreateWalletNoticeScreen(
            controller: _onboarding,
            onContinue: _onboarding.confirmRecoveryNotice,
            onBack: () {},
          ),
        OnboardingState.authSetup => AuthSetupScreen(
            controller: _onboarding,
            onComplete: _onboarding.completeAuthSetup,
          ),
        OnboardingState.ready => _readyShell(),
      },
    );
  }

  Widget _readyShell() {
    return WalletShell(
      home: HomeShell(
        bridge: _bridge,
        receive: _receiveController,
        draft: _draftController,
      ),
      settings: SettingsShell(appState: _appState),
      selectedIndex: _appState.selectedDestination,
      onSelect: _appState.selectDestination,
    );
  }
}

class HomeShell extends StatelessWidget {
  const HomeShell({
    super.key,
    required this.bridge,
    required this.receive,
    required this.draft,
  });

  final BridgeFacade bridge;
  final ReceiveController receive;
  final SendDraftController draft;

  @override
  Widget build(BuildContext context) {
    final accounts = bridge.refreshAccounts();
    return Scaffold(
      body: ListView(
        padding: const EdgeInsets.all(WalletSpacing.l), // ignore: prefer_const_constructors
        children: [
          for (final account in accounts)
            ListTile(
              title: Text(account.name),
              subtitle: Text('${account.network} · ${account.balance}'),
            ),
          const SizedBox(height: WalletSpacing.l),
          FilledButton(
            onPressed: () => Navigator.of(context).push<void>(
              MaterialPageRoute(builder: (_) => ReceiveScreen(controller: receive)),
            ),
            child: const Text('Open receive'),
          ),
          const SizedBox(height: WalletSpacing.s),
          FilledButton(
            onPressed: () => Navigator.of(context).push<void>(
              MaterialPageRoute(builder: (_) => SendScreen(controller: draft, onReview: (_) {})),
            ),
            child: const Text('Send'),
          ),
          const SizedBox(height: WalletSpacing.s),
          FilledButton(
            onPressed: () => Navigator.of(context).push<void>(
              MaterialPageRoute(
                builder: (_) => const ReviewScreen(
                  chain: ChainId.ethereum,
                  recipient: '0x0000000000000000000000000000000000000000',
                  amount: '0.05 ETH',
                  fee: '0.0004 ETH',
                ),
              ),
            ),
            child: const Text('Review placeholder'),
          ),
        ],
      ),
    );
  }
}

class SettingsShell extends StatelessWidget {
  const SettingsShell({super.key, required this.appState});

  final AppState appState;

  @override
  Widget build(BuildContext context) {
    return SettingsScreen(
      removeButtonFocusNode: FocusNode(),
      onLock: () async {},
    );
  }
}
