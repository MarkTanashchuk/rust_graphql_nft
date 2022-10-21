use super::common::{Attributes, Media};
use super::MetadataFormat;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize, SimpleObject)]
/// Example of basic `OpenSea` metadata format
///
/// You can read more about metadata here: <https://docs.opensea.io/docs/metadata-standards>
pub struct Metadata {
    /// Required Name
    pub name: String,

    /// Required Description
    pub description: String,

    /// Required Image URL
    pub image: String,

    /// Optional External URL
    pub external_url: Option<String>,

    /// Optional Attributes
    pub attributes: Option<Vec<Attributes>>,
}

/// Example of basic `OpenSea` metadata format with enriched image field
#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct MetadataPostProcessed {
    /// Required Name
    pub name: String,

    /// Required Description
    pub description: String,

    /// Required Image field with additional metadata such as width and height
    pub image: Media,

    /// Optional External URL
    pub external_url: Option<String>,

    /// Optional Attributes
    pub attributes: Option<Vec<Attributes>>,
}

impl From<Metadata> for MetadataPostProcessed {
    fn from(metadata: Metadata) -> Self {
        Self {
            name: metadata.name,
            description: metadata.description,
            image: Media::new(metadata.image, 0, 0),
            external_url: metadata.external_url,
            attributes: metadata.attributes,
        }
    }
}

impl From<MetadataPostProcessed> for Metadata {
    fn from(metadata: MetadataPostProcessed) -> Self {
        Self {
            name: metadata.name,
            description: metadata.description,
            image: metadata.image.url,
            external_url: metadata.external_url,
            attributes: metadata.attributes,
        }
    }
}

impl MetadataFormat<Self> for Metadata {}
impl MetadataFormat<MetadataPostProcessed> for Metadata {}
