use super::common::Attributes;
use super::MetadataFormat;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize, SimpleObject)]
/// Default metadata format with mandatory fields only according to ERC721 standard (name, image, description)
pub struct Metadata {
    /// Required Name
    pub name: String,

    /// Image URL
    pub image: String,

    /// Required Description
    pub description: String,

    /// Optional External URL
    pub external_url: Option<String>,

    /// Optional Attributes
    pub attributes: Option<Vec<Attributes>>,

    /// Other fields
    #[serde(flatten)]
    #[graphql(skip)]
    pub other: serde_json::Map<String, serde_json::Value>,
}

impl MetadataFormat<Self> for Metadata {}
