use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Attributes {
    pub trait_type: String,
    pub value: String,
}
