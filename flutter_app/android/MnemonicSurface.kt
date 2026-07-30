package com.example.wallet_sample

import android.content.Context
import android.text.InputType
import android.view.View
import android.widget.EditText
import io.flutter.plugin.platform.PlatformView

/**
 * A Flutter platform view that hosts an EditText configured for secure
 * recovery-phrase (mnemonic) entry.
 *
 * Secure-display semantics on Android:
 * Unlike iOS, the platform does NOT automatically obscure secure fields in
 * the recents/thumbnail snapshot. The host Activity is responsible for
 * applying screenshot-suppress and clipboard-clearing semantics. In the
 * Activity's `onCreate` we must call:
 *
 *   window.setFlags(
 *       WindowManager.LayoutParams.FLAG_SECURE,
 *       WindowManager.LayoutParams.FLAG_SECURE
 *   )
 *   window.setShowWhenLocked(true)
 *
 * `FLAG_SECURE` blocks both screenshots and the recents-list snapshot of
 * this Activity, preventing recovery phrases from being captured when the
 * user switches apps. The EditText is configured with `TYPE_TEXT_VARIATION_PASSWORD`
 * to opt into the secure input path and disable suggestions.
 */
class MnemonicSurface(private val context: Context) : PlatformView {
    private val editText = EditText(context)

    init {
        editText.inputType =
            InputType.TYPE_CLASS_TEXT or InputType.TYPE_TEXT_VARIATION_PASSWORD
        editText.isFocusable = true
        editText.setShowSoftInputOnFocus(true)
    }

    override fun getView(): View {
        return editText
    }

    override fun dispose() {}
}
