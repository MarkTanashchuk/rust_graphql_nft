/// Token related API and metadata
pub mod token;

use super::contract::ContractEth;
use async_graphql::{FieldResult, MergedObject, Object};
use token::{TokenDefault, TokenSchemaAPI};

/// Root schema that describes all available fields
///
/// To add a new `GraphQL` field, create a new structure with its own impl and async functions
///
/// Read more here: <https://async-graphql.github.io/async-graphql/en/introduction.html>
#[derive(MergedObject, Default)]
pub struct QueryRoot(pub QueryEthereumToken);

/// Ethereum token scheme with methods for fetching token data
#[derive(Default)]
pub struct QueryEthereumToken;

#[Object(cache_control(max_age = 120))]
impl QueryEthereumToken {
    /// Get all tokens at the Ethereum address (0x...) and the specified range of IDs(offset, limit)
    async fn eth_tokens_with_offset(
        &self,
        address: String,
        offset: usize,
        limit: usize,
    ) -> FieldResult<Vec<TokenDefault>> {
        TokenDefault::tokens(address, Some(offset), Some(limit)).await
    }

    /// Get all tokens at Ethereum address (0x...)
    async fn eth_tokens(&self, address: String) -> FieldResult<Vec<TokenDefault>> {
        TokenDefault::tokens(address, None, None).await
    }

    /// Get one token at Ethereum address (0x...) and its ID
    async fn eth_token(&self, address: String, id: usize) -> FieldResult<TokenDefault> {
        TokenDefault::tokens(address, Some(id), Some(id))
            .await?
            .pop()
            .ok_or(async_graphql::Error {
                message: String::from("Token not found"),
                source: None,
                extensions: None,
            })
    }
}
