import 'package:flutter/material.dart';

import '../theme/tokens.dart';

class AmountInput extends StatelessWidget {
  const AmountInput({
    super.key,
    required this.controller,
    required this.validator,
    required this.onMax,
    this.onChanged,
  });

  final TextEditingController controller;
  final FormFieldValidator<String> validator;
  final VoidCallback onMax;
  final ValueChanged<String>? onChanged;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: WalletSpacing.s),
      child: TextFormField(
        controller: controller,
        keyboardType: const TextInputType.numberWithOptions(decimal: true),
        decoration: InputDecoration(
          labelText: 'Amount',
          suffixIcon: TextButton(onPressed: onMax, child: const Text('Max')),
        ),
        validator: validator,
        onChanged: onChanged,
      ),
    );
  }
}
