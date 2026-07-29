use rpc_client::classify::classify;

#[test]
fn html_body_with_nonce_substring_does_not_misclassify_as_chain_state() {
    assert_eq!(
        classify("HTTP/1.1 200 OK\nContent-Type: text/html\n<!-- nonce: abc -->"),
        rpc_client::classify::Outcome::Unknown
    );
}

#[test]
fn structured_codes_take_precedence() {
    assert_eq!(
        classify(r#"{"error":{"code":-32000,"message":"nonce too low"}}"#),
        rpc_client::classify::Outcome::ChainState
    );
}
