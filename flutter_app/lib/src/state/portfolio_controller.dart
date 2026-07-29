import 'package:flutter/foundation.dart';

import '../bridge/bridge_facade_stub.dart';

enum PortfolioViewState { loading, ready, stale, offline, error }

class PortfolioController extends ChangeNotifier {
  PortfolioController({required BridgeFacade bridge}) : _bridge = bridge;

  final BridgeFacade _bridge;
  PortfolioViewState _state = PortfolioViewState.loading;
  List<ChainDescriptor> _chains = const [];

  PortfolioViewState get state => _state;
  List<ChainDescriptor> get chains => List.unmodifiable(_chains);

  Future<void> refresh() async {
    _state = PortfolioViewState.loading;
    notifyListeners();
    try {
      _chains = _bridge.refreshAccounts();
      _state = PortfolioViewState.ready;
    } on BridgeException {
      _state = PortfolioViewState.error;
    }
    notifyListeners();
  }

  void markStale() {
    _state = PortfolioViewState.stale;
    notifyListeners();
  }

  void markOffline() {
    _state = PortfolioViewState.offline;
    notifyListeners();
  }
}
