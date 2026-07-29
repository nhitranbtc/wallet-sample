import 'package:flutter/foundation.dart';

import '../bridge/bridge_facade_stub.dart';

class SendDraftController extends ChangeNotifier {
  SendDraftController({
    this.availableBalance = BigInt.zero,
    this.chain = ChainId.ethereum,
  });

  BigInt availableBalance;
  ChainId chain;
  String recipient = '';
  String amount = '';
  String fee = 'Fee refresh required';

  String? validate(String value) {
    if (value.trim().isEmpty) {
      return 'Enter an amount';
    }
    final parsed = BigInt.tryParse(value.trim());
    if (parsed == null) {
      return 'Enter a whole-number amount';
    }
    if (parsed <= BigInt.zero) {
      return 'Amount must be greater than zero';
    }
    if (availableBalance > BigInt.zero && parsed > availableBalance) {
      return 'Amount exceeds available balance';
    }
    return null;
  }

  void selectChain(ChainId value) {
    chain = value;
    fee = 'Fee refresh required';
    notifyListeners();
  }

  void updateRecipient(String value) {
    recipient = value;
    notifyListeners();
  }

  void updateAmount(String value) {
    amount = value;
    notifyListeners();
  }

  void useMax() {
    amount = availableBalance.toString();
    notifyListeners();
  }

  Future<void> refreshFee() async {
    fee = chain == ChainId.ethereum ? '0.0004 ETH' : '0.00001 BTC';
    notifyListeners();
  }
}
