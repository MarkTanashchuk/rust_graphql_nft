use super::common::Attributes;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct Metadata {
    pub name: String,
    pub image: String,
    pub description: String,
    pub data: Option<String>,
    pub external_url: Option<String>,
    pub attributes: Option<Vec<Attributes>>,

    #[serde(flatten)]
    #[graphql(skip)]
    pub other: serde_json::Map<String, serde_json::Value>,
}
