use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
/// Enriched image field with additional metadata
pub struct Media {
    /// URL
    pub url: String,

    /// Width
    pub width: i32,

    /// Height
    pub height: i32,
}

impl Media {
    /// Create new instance of `Media`
    #[must_use]
    pub const fn new(url: String, width: i32, height: i32) -> Self {
        Self { url, width, height }
    }
}
