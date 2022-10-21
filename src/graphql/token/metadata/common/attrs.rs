use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
/// Read more about attributes here: <https://docs.opensea.io/docs/metadata-standards#attributes>
pub struct Attributes {
    /// Trait type
    pub trait_type: String,

    /// Value
    pub value: String,
}
