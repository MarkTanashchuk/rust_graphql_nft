use super::{normalize_ipfs_url, ContractGetter};
use web3::{
    contract,
    types::{self, U256},
};

#[async_trait::async_trait]
pub trait QueryAPI
where
    Self: ContractGetter + Sized,
{
    /// Executе a custom contract method
    ///
    /// # Errors
    /// Returns `Err` if the contract method execution failed
    ///
    /// # Examples
    /// ```
    /// let contract = ContractEth::new("0x...".to_string()).await?;
    /// let result = contract.query("method_name", (param1, param2)).await?;
    /// ```
    async fn query<R, P>(&self, command: &str, params: P) -> Result<R, contract::Error>
    where
        R: contract::tokens::Detokenize,
        P: contract::tokens::Tokenize + Send,
    {
        self.contract()
            .query(command, params, None, contract::Options::default(), None)
            .await
    }

    /// Get the token URI
    ///
    /// # Errors
    /// Returns `Err` if `self.query` failed or if ipfs://<token URI> cannot be loaded by any of the 4 gateways
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// use blockchain::contract::ContractEth;
    ///
    /// let contract = ContractEth::new("0x...".to_string())?;
    /// let token_uri = contract.token_uri(1).await?;
    /// ```
    async fn token_uri(&self, token_id: types::U256) -> Result<String, contract::Error> {
        let token_uri = self.query("tokenURI", token_id).await?;
        let token_uri = normalize_ipfs_url(token_uri).await?;

        Ok(token_uri)
    }

    /// Get the total amount of tokens
    ///
    /// # Errors
    /// Returns `Err` if `self.query` failed or if ipfs://<token URI> cannot be loaded by any of the 4 gateways
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// use blockchain::contract::ContractEth;
    ///
    /// let contract = ContractEth::new("0x...".to_string())?;
    /// let total_supply = contract.total_supply().await?;
    /// ```
    async fn total_supply(&self) -> Result<U256, contract::Error> {
        let total_supply = self.query("totalSupply", ()).await?;

        Ok(total_supply)
    }
}
