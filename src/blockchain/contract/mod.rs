use super::address::{Eth, Sol};
use super::ipfs::normalize_ipfs_url;
use std::sync::Arc;
use std::{env, str::FromStr};
use web3::{
    contract::{
        tokens::{Detokenize, Tokenize},
        Contract, Options,
    },
    types::Address,
};

const ABI: &[u8; 6172] = include_bytes!("./abi.json");

/// Wrapper around web3 contract for simplified usage
#[derive(Debug, Clone)]
pub struct CustomContract {
    pub inner: Arc<Contract<web3::transports::Http>>,
}

impl<'a> CustomContract {
    fn create_web3() -> web3::Web3<web3::transports::Http> {
        let transport_url = env::var("TRANSPORT_URL").expect("TRANSPORT_URL in .env must be set");
        let http = web3::transports::Http::new(&transport_url[..]).expect("Invalid TRANSPORT_URL");
        web3::Web3::new(http)
    }

    /// Create a new Ethereum contract instance
    pub fn new_eth(address: &str) -> Self {
        let web3 = Self::create_web3();

        if let Ok(address) = Eth::new(address) {
            let address = Address::from_str(address.value).expect("Address always valid");

            Self {
                inner: Arc::new(
                    Contract::from_json(web3.eth(), address, ABI).expect("Invalid contract"),
                ),
            }
        } else {
            panic!("Invalid Ethereum address")
        }
    }

    /// Create a new Solana contract instance
    pub fn new_sol(address: &str) -> Self {
        let web3 = Self::create_web3();

        if let Ok(address) = Sol::new(address) {
            let address = Address::from_str(address.value).expect("Address always valid");

            Self {
                inner: Arc::new(
                    Contract::from_json(web3.eth(), address, ABI).expect("Invalid contract"),
                ),
            }
        } else {
            panic!("Invalid Solana address")
        }
    }

    pub async fn query<R, P>(&'a self, command: &str, params: P) -> Result<R, web3::contract::Error>
    where
        R: Detokenize + 'a,
        P: Tokenize + 'a,
    {
        self.inner
            .query(command, params, None, Options::default(), None)
            .await
    }

    /// Fetch token URI
    pub async fn query_token_uri(
        &'a self,
        token_id: web3::types::U256,
    ) -> Result<String, web3::contract::Error> {
        let token_uri = self.query("tokenURI", token_id).await?;
        let token_uri = normalize_ipfs_url(token_uri).await?;

        Ok(token_uri)
    }
}
