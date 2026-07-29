use rpc_client::{Chain, EndpointConfig, Network, ProviderPolicy};

fn cfg(url: &str, net: Network) -> EndpointConfig {
    EndpointConfig {
        chain: Chain::Ethereum,
        url: url.into(),
        network: net,
        policy: ProviderPolicy::dev_default(Chain::Ethereum),
    }
}

#[test]
fn rejects_http() {
    assert!(cfg("http://example.com", Network::Testnet)
        .validate()
        .is_err());
}

#[test]
fn rejects_mainnet() {
    assert!(cfg("https://mainnet.example", Network::Mainnet)
        .validate()
        .is_err());
}

#[test]
fn rejects_off_allowlist_host() {
    let c = cfg("https://attacker.example", Network::Testnet);
    assert!(c.validate().is_err());
}

#[test]
fn accepts_on_allowlist_host() {
    let c = cfg("https://rpc.sepolia.org", Network::Testnet);
    assert!(c.validate().is_ok());
}
