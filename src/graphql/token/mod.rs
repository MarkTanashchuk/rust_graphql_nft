mod metadata;

use async_graphql::{ComplexObject, Context, OutputType, SimpleObject};
use serde::{Deserialize, Serialize};

pub type TokenDefault = TokenSchema<metadata::default::Metadata>;
pub type TokenOpenSea = TokenSchema<metadata::opensea::Metadata>;

#[derive(Debug, Serialize, SimpleObject)]
#[graphql(complex)]
pub struct TokenSchema<T>
where
    for<'de> T: OutputType + Deserialize<'de>,
{
    #[graphql(name = "tokenId")]
    pub id: String,

    #[graphql(name = "tokenURI")]
    pub uri: String,

    #[graphql(name = "tokenAddress")]
    pub address: String,

    #[graphql(skip)]
    #[serde(skip)]
    pub _metadata: std::marker::PhantomData<T>,

    #[graphql(skip)]
    #[serde(skip)]
    pub client: reqwest::Client,
}

impl<T> TokenSchema<T>
where
    for<'de> T: OutputType + Deserialize<'de>,
{
    fn process(&self, metadata: T) -> async_graphql::Result<T> {
        Ok(metadata)
    }
}

#[ComplexObject]
impl<T> TokenSchema<T>
where
    for<'de> T: OutputType + Deserialize<'de>,
{
    async fn metadata(&self, _ctx: &Context<'_>) -> async_graphql::Result<T> {
        let response = self.client.get(&self.uri).send().await?;

        self.process(response.json().await?)
    }
}

impl<T> TokenSchema<T>
where
    for<'de> T: OutputType + Deserialize<'de>,
{
    pub fn new(id: String, uri: String, address: String, client: reqwest::Client) -> Self {
        Self {
            id,
            uri,
            address,
            _metadata: std::marker::PhantomData,
            client,
        }
    }
}
