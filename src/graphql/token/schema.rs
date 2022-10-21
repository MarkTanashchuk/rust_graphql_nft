use crate::contract::CustomContract;

use super::metadata::MetadataFormat;

use async_graphql::{FieldResult, Result, SimpleObject};
use futures::future::join_all;
use reqwest::Client;
use serde::Serialize;
use std::marker::PhantomData;
use web3::types::U256;

#[async_trait::async_trait]
/// API for receiving one or more tokens and their metadata
pub trait TokenSchemaAPI<PreMetadata, PostMetadata>
where
    Self: Sized,
    PreMetadata: MetadataFormat<PostMetadata>,
    PostMetadata: MetadataFormat<PreMetadata>,
{
    /// Create a new token instance
    async fn new(id: String, uri: String, address: String, client: Client) -> Result<Self>;

    /// Get all tokens at the Ethereum address (0x...) and the specified range of IDs(offset, limit)
    async fn tokens(
        address: String,
        offset: Option<usize>,
        limit: Option<usize>,
    ) -> FieldResult<Vec<Self>>;

    /// Get token metadata and return it in the processed format
    async fn get_metadata(
        uri: &str,
        client: reqwest::Client,
    ) -> async_graphql::Result<PostMetadata> {
        let response = client.get(uri).send().await?;
        let metadata: PreMetadata = response.json().await?;
        let metadata: PostMetadata = metadata.postprocess();

        Ok(metadata)
    }
}

#[derive(Debug, Serialize, Clone, SimpleObject)]
/// ERC721-compliant token scheme
pub struct TokenSchema<PreMetadata, PostMetadata, Contract>
where
    PreMetadata: MetadataFormat<PostMetadata>,
    PostMetadata: MetadataFormat<PreMetadata>,
    Contract: CustomContract,
{
    /// Token ID
    #[graphql(name = "tokenId")]
    pub id: String,

    /// Token URI
    #[graphql(name = "tokenURI")]
    pub uri: String,

    /// Contract address
    #[graphql(name = "tokenAddress")]
    pub address: String,

    /// Token metadata
    #[graphql(name = "tokenMetadata")]
    pub metadata: PostMetadata,

    /// Phantom data to prevent the compiler from complaining about unused generic types
    #[graphql(skip)]
    #[serde(skip)]
    pub _metadata: (
        PhantomData<PreMetadata>,
        PhantomData<PostMetadata>,
        PhantomData<Contract>,
    ),
}

#[async_trait::async_trait]
impl<PreMetadata, PostMetadata, Contract> TokenSchemaAPI<PreMetadata, PostMetadata>
    for TokenSchema<PreMetadata, PostMetadata, Contract>
where
    PreMetadata: MetadataFormat<PostMetadata>,
    PostMetadata: MetadataFormat<PreMetadata>,
    Contract: CustomContract,
{
    /// Create a new token and get its metadata
    async fn new(id: String, uri: String, address: String, client: Client) -> Result<Self> {
        let metadata = Self::get_metadata(&uri, client).await?;

        Ok(Self {
            id,
            uri,
            address,
            metadata,
            _metadata: (PhantomData, PhantomData, PhantomData),
        })
    }

    /// Concurrently fetch all tokens at the Ethereum address (0x...)
    async fn tokens(
        address: String,
        offset: Option<usize>,
        limit: Option<usize>,
    ) -> FieldResult<Vec<Self>> {
        let client = Client::new();
        let contract = Contract::new(address.clone())?;

        let token_ids = {
            let batch = 20;
            let offset = offset.unwrap_or(1);
            let limit = {
                let total_token_amount = contract.total_supply().await?.as_usize();
                limit.unwrap_or(total_token_amount)
            };

            let mut ids: Vec<Vec<U256>> = Vec::with_capacity(limit);
            let mut id = offset;
            while ids.capacity() >= id {
                let limit = std::cmp::min(limit, id + batch);
                ids.push((id..=limit).map(U256::from).collect());
                id += batch;
            }
            ids
        };

        let tokens = join_all(token_ids.into_iter().map(|ids| {
            join_all(ids.into_iter().map(|id| {
                let contract = contract.clone();
                let client = client.clone();
                let address = address.clone();

                async move {
                    let uri = contract.token_uri(id).await?;
                    let id = id.to_string();

                    Self::new(id, uri, address, client).await
                }
            }))
        }));
        let tokens = tokens
            .await
            .into_iter()
            .flatten()
            .collect::<Result<Vec<_>, _>>();

        Ok(tokens?)
    }
}
