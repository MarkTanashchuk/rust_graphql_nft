#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![forbid(unsafe_code)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::unused_async)]
#![doc = include_str!("../README.md")]

/// Contains API for creating and interacting with Ethereum contracts
pub mod contract;

/// Contains GraphQL [fields](graphql), [token schema](graphql::token) and [metadata formats](graphql::token::metadata)
pub mod graphql;

/// Contains [server](server) and browser-related functionality
pub mod server;

use graphql::QueryRoot;
use server::start_server;

/// JSON ABI representation of ERC721 contract (<https://eips.ethereum.org/EIPS/eip-721>)
pub const ABI: &[u8; 8374] = include_bytes!("./erc721_abi.json");

/// Main function that starts the server
///
/// # Errors
/// Returns an error if a problem with [`actix_web::HttpServer`] server occurs
pub fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let port = std::env::var("PORT").unwrap_or_else(|_| 8080.to_string());

    start_server(port)?;

    Ok(())
}
