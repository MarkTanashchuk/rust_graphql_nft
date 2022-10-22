use async_graphql::OutputType;
use serde::de::DeserializeOwned;

/// Commonly used data fields such as [attributes](common::Attributes), [media](common::Media), etc
pub mod common;

/// Default metadata format
pub mod default;

/// Example of basic `OpenSea` metadata format
pub mod opensea;

/// Defines what metadata will be converted into
pub trait MetadataFormat<PostProcessed>
where
    Self: OutputType + DeserializeOwned + From<PostProcessed>,
    PostProcessed: OutputType + DeserializeOwned + From<Self>,
{
    /// Converts metadata into post-processed format
    fn postprocess(self) -> PostProcessed {
        self.into()
    }
}
