use super::common::Attributes;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub image: String,
    pub external_url: Option<String>,
    pub attributes: Option<Vec<Attributes>>,
}
