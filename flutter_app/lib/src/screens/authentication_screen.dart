import 'package:flutter/material.dart';

import '../theme/tokens.dart';
import '../widgets/status_timeline.dart';

class AuthenticationScreen extends StatelessWidget {
  const AuthenticationScreen({super.key, this.message = 'Awaiting biometric prompt'});

  final String message;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Authentication')),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: WalletBreakpoints.contentMax),
          child: Padding(
            padding: const EdgeInsets.all(WalletSpacing.xxl),
            child: LiveRegion(
              label: message,
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  const SizedBox.square(
                    dimension: WalletSpacing.xxxl * 1.5,
                    child: CircularProgressIndicator(),
                  ),
                  const SizedBox(height: WalletSpacing.xl),
                  Text(message, style: Theme.of(context).textTheme.titleMedium),
                  const SizedBox(height: WalletSpacing.m),
                  const Text(
                    'No action is required in the app. '
                    'The platform biometric sheet drives the result.',
                    textAlign: TextAlign.center,
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
