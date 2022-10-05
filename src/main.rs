#![deny(missing_docs)]

//! # Rust GraphQL Server for searching NFT (meta)data

mod blockchain;
mod graphql;
mod server;

use graphql::QueryRoot;
use server::start_server;

/// Initializes the server and starts it
fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let port = std::env::var("PORT").unwrap_or_else(|_| 8080.to_string());

    start_server(port)?;

    Ok(())
}
