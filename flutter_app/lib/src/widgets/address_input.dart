import 'package:flutter/material.dart';

import '../theme/tokens.dart';

class AddressInput extends StatelessWidget {
  const AddressInput({
    super.key,
    required this.controller,
    this.onChanged,
    this.validator,
  });

  final TextEditingController controller;
  final ValueChanged<String>? onChanged;
  final FormFieldValidator<String>? validator;

  @override
  Widget build(BuildContext context) {
    return Semantics(
      textField: true,
      label: 'Recipient wallet address',
      child: Padding(
        padding: const EdgeInsets.symmetric(vertical: WalletSpacing.s),
        child: TextFormField(
          controller: controller,
          autocorrect: false,
          enableSuggestions: false,
          decoration: const InputDecoration(labelText: 'Recipient address'),
          onChanged: onChanged,
          validator: validator,
        ),
      ),
    );
  }
}
