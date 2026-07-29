import 'package:flutter/foundation.dart';

class AppState extends ChangeNotifier {
  int _selectedDestination = 0;
  bool _locked = false;

  int get selectedDestination => _selectedDestination;
  bool get locked => _locked;

  void selectDestination(int index) {
    _selectedDestination = index;
    notifyListeners();
  }

  void showHome() {
    _selectedDestination = 0;
    _locked = false;
    notifyListeners();
  }

  void showLocked() {
    _locked = true;
    notifyListeners();
  }
}
