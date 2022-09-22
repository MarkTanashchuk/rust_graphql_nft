use async_graphql::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Attributes {
    pub trait_type: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct Image {
    image_type: String,
    url: String,
    width: i32,
    height: i32,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct Metadata {
    pub url: String,
    pub mime_type: String,
    pub height: i32,
    pub width: i32,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct TokenMetadata {
    pub name: String,
    pub description: String,
    pub external_url: String,
    pub attributes: Vec<Attributes>,
    pub image: Metadata,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct Token {
    pub metadata: Option<TokenMetadata>,

    #[graphql(name = "tokenURI")]
    pub token_uri: String,

    #[graphql(name = "tokenId")]
    pub token_id: String,

    #[graphql(name = "tokenAddress")]
    pub token_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub name: String,
    pub description: String,
    pub image: String,
    pub media: Media,
    pub tags: Vec<String>,
    pub attributes: Vec<Attributes>,

    #[serde(rename = "createdBy")]
    pub created_by: String,

    #[serde(rename = "yearCreated")]
    pub year_created: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Media {
    pub uri: String,
    pub dimensions: String,
    pub size: String,

    #[serde(rename = "mimeType")]
    pub mime_type: String,
}
