mod token;

use crate::blockchain::CustomContract;
use async_graphql::{FieldResult, Object};
use token::TokenDefault;
use web3::types::U256;

/// Root schema that describes all available fields
pub struct QueryRoot;

#[Object(cache_control(max_age = 60))]
impl QueryRoot {
    /// Get all the tokens by Ethereum address (0x...) and provided ID range
    /// To get all tokens from address, set `from` to 1 and `to` to 999
    async fn eth_tokens(
        &self,
        address: String,
        from: i32,
        to: i32,
    ) -> FieldResult<Vec<TokenDefault>> {
        let mut tokens = Vec::with_capacity(to as usize);
        let contract = CustomContract::new_eth(&address);
        let client = reqwest::Client::new();

        for i in from..to {
            println!("Checking token at id: {i}");

            if let Ok(uri) = contract.clone().query_token_uri(U256::from(i)).await {
                tokens.push(TokenDefault::new(
                    i.to_string(),
                    uri,
                    address.clone(),
                    client.clone(),
                ));
            } else {
                println!("Token at id: {i} not found");
                break;
            }
        }

        Ok(tokens)
    }

    /// Get single token by Ethereum address (0x...) and its ID
    async fn eth_token(&self, address: String, id: String) -> FieldResult<TokenDefault> {
        let client = reqwest::Client::new();
        let uri = {
            let id = U256::from_dec_str(&id)?;
            let contract = CustomContract::new_eth(&address);
            contract.query_token_uri(id).await?
        };

        Ok(TokenDefault::new(id, uri, address, client.clone()))
    }
}
