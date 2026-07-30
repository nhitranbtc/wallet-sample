use rpc_client::EndpointConfig;
use wallet_domain::account::Network;

/// Configuration for [`crate::EthereumAdapter`].
///
/// `chain_id` is the EIP-155 chain id used when building transactions;
/// the proof targets Sepolia (`11155111`). `endpoint` carries the RPC
/// URL plus the provider policy that `EndpointConfig::validate` enforces.
#[derive(Debug, Clone)]
pub struct EthereumConfig {
    pub network: Network,
    pub endpoint: EndpointConfig,
    pub chain_id: u64,
}
