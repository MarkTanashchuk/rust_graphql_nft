# Rust GraphQL Server for searching NFT (meta)data

## Prerequisites

1. [Install Rust](https://www.rust-lang.org/tools/install) if it is not already installed.

```cli
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

1. Clone the repo

```cli
git clone https://github.com/MarkTanashchuk/rust_graphql_nft.git && cd ./rust_graphql_nft
```

### Usage

1. Start server

```cli
cargo run --release
```

1. Open [localhost:8080](http://localhost:8080)

### Example

```graphql
query GetTokenWithMetadata($tokenAddress: String!, $tokenId: String!) {
  token(tokenAddress: $tokenAddress, tokenId: $tokenId) {
    tokenAddress
    tokenId
    tokenURI
    metadata {
      name
      description
      image {
        url
        width
        height
      }
      attributes {
        traitType
        value
      }
    }
  }
}
```

Variables

```json
{
  "tokenAddress": "0x7f371bed0bdb2012c01f219ca1c4cbcb35f37aef",
  "tokenId": "2"
}
```

Currently, only the Ethereum address format is supported

## Roadmap

- Solana support
- IPFS support
- More support for metadata extensions
- Performance optimizations
- MIME in `enum` format
- Site visualization
- Authorization
- Benchmarks
- Rust API
