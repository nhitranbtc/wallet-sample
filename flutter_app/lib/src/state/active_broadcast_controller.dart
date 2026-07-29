import 'package:flutter/foundation.dart';

import '../bridge/bridge_facade_stub.dart';
import 'biometric_gate.dart';

// Stages: preparing | awaitingAuth | signing | broadcasting | submitted | confirmed | failed.
// The brief Step 8 wording said "six" but enumerates seven; this is the canonical seven-stage pipeline.
enum TransferStage {
  preparing,
  awaitingAuth,
  signing,
  broadcasting,
  submitted,
  confirmed,
  failed,
}

extension TransferStageLabel on TransferStage {
  String get label => switch (this) {
        TransferStage.preparing => 'Preparing',
        TransferStage.awaitingAuth => 'Awaiting authentication',
        TransferStage.signing => 'Signing',
        TransferStage.broadcasting => 'Broadcasting',
        TransferStage.submitted => 'Submitted',
        TransferStage.confirmed => 'Confirmed',
        TransferStage.failed => 'Failed',
      };
}

class ActiveBroadcastController extends ChangeNotifier {
  ActiveBroadcastController({
    required BridgeFacade bridge,
    required BiometricGate gate,
  })  : _bridge = bridge,
        _gate = gate;

  final BridgeFacade _bridge;
  final BiometricGate _gate;
  TransferStage _stage = TransferStage.preparing;
  String? _transactionHash;

  TransferStage get stage => _stage;
  String? get transactionHash => _transactionHash;

  Future<void> authenticateAndBroadcast(PreparedHandle prepared) async {
    _setStage(TransferStage.awaitingAuth);
    final authenticated = await _gate.promptForSigning();
    if (!authenticated) {
      _setStage(TransferStage.failed);
      throw StateError('signing requires biometric authentication');
    }
    _setStage(TransferStage.signing);
    _setStage(TransferStage.broadcasting);
    try {
      _transactionHash = await _bridge.authenticateSignAndBroadcast(
        prepared: prepared,
      );
      _setStage(TransferStage.submitted);
    } on BridgeException {
      _setStage(TransferStage.failed);
      rethrow;
    }
  }

  void refreshStatus() {
    final hash = _transactionHash;
    if (hash == null) return;
    final status = _bridge.watchTransferStatus(hash);
    switch (status) {
      case TransactionStatus.Confirmed:
        _setStage(TransferStage.confirmed);
      case TransactionStatus.Failed:
        _setStage(TransferStage.failed);
      case TransactionStatus.Pending:
      case TransactionStatus.Unknown:
        _setStage(TransferStage.submitted);
    }
  }

  void _setStage(TransferStage value) {
    _stage = value;
    notifyListeners();
  }
}
