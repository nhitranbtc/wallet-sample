use serde::{Deserialize, Serialize};
use url::Url;
use wallet_domain::account::{ChainId, Network};
use wallet_domain::error::ChainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Chain {
    Ethereum,
    Bitcoin,
    Solana,
    Tron,
}

impl From<ChainId> for Chain {
    fn from(chain: ChainId) -> Self {
        match chain {
            ChainId::Ethereum => Self::Ethereum,
            ChainId::Bitcoin => Self::Bitcoin,
            ChainId::Solana => Self::Solana,
            ChainId::Tron => Self::Tron,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderPolicy {
    pub allowed_hosts: Vec<String>,
    pub spki_pins: Vec<String>,
}

impl ProviderPolicy {
    pub fn dev_default(chain: Chain) -> Self {
        let allowed_hosts = match chain {
            Chain::Ethereum => vec!["rpc.sepolia.org".into(), "sepolia.drpc.org".into()],
            Chain::Bitcoin => vec!["esplora.testnet.example".into()],
            Chain::Solana => vec!["api.devnet.solana.com".into()],
            Chain::Tron => vec!["api.shasta.trongrid.io".into()],
        };

        Self {
            allowed_hosts,
            spki_pins: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointConfig {
    pub chain: Chain,
    pub url: String,
    pub network: Network,
    pub policy: ProviderPolicy,
}

impl EndpointConfig {
    pub fn validate(&self) -> Result<(), ChainError> {
        if self.network == Network::Mainnet {
            return Err(ChainError::Configuration(
                "mainnet is not permitted in the architecture proof".into(),
            ));
        }

        let parsed =
            Url::parse(&self.url).map_err(|_| ChainError::Configuration("invalid url".into()))?;
        if parsed.scheme() != "https" {
            return Err(ChainError::Configuration("https required".into()));
        }

        let host = parsed
            .host_str()
            .ok_or_else(|| ChainError::Configuration("missing host".into()))?;
        if !self
            .policy
            .allowed_hosts
            .iter()
            .any(|allowed| allowed == host)
        {
            return Err(ChainError::Configuration(format!(
                "host {host} not in allow-list"
            )));
        }

        Ok(())
    }
}
