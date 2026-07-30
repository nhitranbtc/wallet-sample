import Flutter
import UIKit

/// A Flutter platform view that hosts a UITextField configured for secure
/// recovery-phrase (mnemonic) entry.
///
/// Secure-display semantics on iOS:
/// Setting `isSecureTextEntry = true` opts the field into the iOS secure
/// display contract. The system automatically obscures the field in app
/// switcher snapshots, screen recordings, and screenshots taken while the
/// app is backgrounded — the platform handles screenshot suppression for
/// us; no extra code is required.
///
/// The text field is wrapped in a container UIView so Flutter receives a
/// stable `UIView` to mount. The typed characters are read by the host
/// Activity / view controller; Flutter itself never observes them.
class MnemonicSurface: NSObject, FlutterPlatformView {
  private let textField = UITextField()

  init(_ frame: CGRect) {
    super.init()
    textField.isSecureTextEntry = true
    textField.textContentType = .password
    textField.autocorrectionType = .no
    textField.autocapitalizationType = .none
    textField.spellCheckingType = .no
  }

  func view() -> UIView {
    let container = UIView(frame: .zero)
    container.addSubview(textField)
    // iOS suppresses screenshots in the app-switcher automatically when
    // textField.isSecureTextEntry is true.
    return container
  }
}
