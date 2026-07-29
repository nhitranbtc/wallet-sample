import 'package:flutter_test/flutter_test.dart';
import 'package:wallet_sample/src/state/send_draft_controller.dart';

void main() {
  test('amount validation rejects empty and zero but accepts a balance-aware amount', () {
    final controller = SendDraftController(availableBalance: BigInt.from(1000));

    expect(controller.validate(''), 'Enter an amount');
    expect(controller.validate('0'), 'Amount must be greater than zero');
    expect(controller.validate('123'), isNull);
  });
}
