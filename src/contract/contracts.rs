use super::{AddressEth, ContractGetter, CustomContract, QueryAPI};
use crate::ABI;
use anyhow::Result;
use std::str::FromStr;
use std::sync::Arc;
use web3::types;
use web3::{contract, transports};

/// Wrapper around web3 contract with Ethereum-related API
#[derive(Debug, Clone)]
pub struct ContractEth {
    /// Web3 contract
    pub inner: Arc<contract::Contract<transports::Http>>,
}

impl QueryAPI for ContractEth {}
impl CustomContract for ContractEth {
    fn new(address: String) -> Result<Self> {
        AddressEth::new(address.clone())?;
        let address = types::Address::from_str(&address)?;

        let transport = Self::create_transport();
        let contract = contract::Contract::from_json(transport.eth(), address, ABI)?;

        Ok(Self {
            inner: Arc::new(contract),
        })
    }
}

impl ContractGetter for ContractEth {
    fn contract(&self) -> Arc<contract::Contract<transports::Http>> {
        self.inner.clone()
    }
}
