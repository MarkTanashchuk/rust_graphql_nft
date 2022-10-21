mod address;
mod contracts;
mod ipfs;
mod query_api;

use address::AddressEth;
use anyhow::Result;
use ipfs::normalize_ipfs_url;
use query_api::QueryAPI;
use std::env;
use std::sync::Arc;
use web3::{contract, transports, Web3};

pub use contracts::ContractEth;

/// Trait for creating contract wrappers
pub trait CustomContract
where
    Self: QueryAPI + Clone + Send + Sync,
{
    /// Creates a new instance of the contract
    ///
    /// # Errors
    /// Returns `Err` if the address for the selected contract is invalid
    fn new(address: String) -> Result<Self>;

    /// Creates an instance of the Web3 transport
    ///
    /// # Panics
    /// Panics if the `TRANSPORT_URL` environment variable is not set or invalid
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// use blockchain::contract::ContractEth;
    ///
    /// let transport = ContractEth::create_transport();
    /// ```
    #[must_use]
    fn create_transport() -> Web3<transports::Http> {
        let transport_url = env::var("TRANSPORT_URL").expect("TRANSPORT_URL in .env must be set");
        let http = transports::Http::new(&transport_url[..]).expect("Invalid TRANSPORT_URL");
        Web3::new(http)
    }
}

/// Trait for access to the contract instance
pub trait ContractGetter {
    /// Returns the contract instance
    fn contract(&self) -> Arc<contract::Contract<transports::Http>>;
}
