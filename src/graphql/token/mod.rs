/// Module with structures of token metadata
pub mod metadata;

/// Module with Token's main functionality
pub mod schema;

use super::ContractEth;
pub use schema::{TokenSchema, TokenSchemaAPI};

/// Default token schema with default metadata format and Ethereum contract
pub type TokenDefault =
    TokenSchema<metadata::default::Metadata, metadata::default::Metadata, ContractEth>;

/// Example of `OpenSea` token scheme with its metadata format and Ethereum contract
pub type _TokenOpenSea =
    TokenSchema<metadata::opensea::Metadata, metadata::opensea::MetadataPostProcessed, ContractEth>;
