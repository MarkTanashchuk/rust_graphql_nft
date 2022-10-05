# Rust GraphQL Server for searching NFT (meta)data

Please note that this repo is still in active development. This means some of the core API and interfaces might change from one version to another. However, feel free to use it and provide some early feedback if you wish to contribute to the direction of this project.

## Prerequisites

- [Install Rust](https://www.rust-lang.org/tools/install) if it is not already installed.

```cli
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- Clone the repo

```cli
git clone https://github.com/MarkTanashchuk/rust_graphql_nft.git && cd ./rust_graphql_nft
```

- Create [Infura](https://infura.io/) Web3 API Key and copy network endpoint(with https)

- Configure `.env` file, e.g.

```env
TRANSPORT_URL=https://mainnet.infura.io/v3/API_KEY
# PORT=8080 - Optional, 8080 by default
```

## Usage

1. Start server

```cli
cargo run --release
```

2. Open localhost:{PORT}. By default [localhost:8080](http://localhost:8080)

## Get Started

Note: Currently, only the Ethereum address format is supported

### Custom queries

The server is written using [async-graphql](https://async-graphql.github.io/async-graphql/en/quickstart.html), so to add new queries you need to update the graphql/mod.rs file according to docs.

### Custom Metadata 

To add custom metadata formats with custom fields, add a new file with the required structure to the src/graphql/token/metadata folder.

### Example of getting single NFT

```graphql
# Variables
# {
#   "address": "0x7f371bed0bdb2012c01f219ca1c4cbcb35f37aef",
#   "id": "10"
# }

query GetTokenWithMetadata($address: String!, $id: String!) {
  ethToken(address: $address, id: $id) {
    tokenAddress
    tokenId
    tokenURI
    metadata {
      name
      description
      image
      attributes {
        traitType
        value
      }
    }
  }
}
```

### Example of getting multiple NFTs

Returns all tokens in the specified (`from`-`to`) range, if the value of `to` is greater than the total amount of tokens, then all tokens starting from the value of `from` will be returned.

```graphql
# Variables
# {
#   "address": "0x7f371bed0bdb2012c01f219ca1c4cbcb35f37aef",
#   "from": 1,
#   "to": 90
# }

query GetTokenWithMetadata($address: String!, $from: Int!, $to: Int!) {
  ethTokens(address: $address, from: $from, to: $to) {
    tokenAddress
    tokenId
    tokenURI
    metadata {
      name
      description
      image
      attributes {
        traitType
        value
      }
    }
  }
}
```

## Possible errors

- "Api error: code 401", line X, column X

The `TRANSPORT_URL` in the `.env` file is set incorrectly or the provided `API_KEY` does not allow unauthorized access

In case of `infura`, create a new Web3 API, click Manage Key and copy the full Ethereum endpoint into `TRANSPORT_URL` in the `.env` file.

## Roadmap

- Solana support
- Caching
- Complete support of WebSockets
- Pagination
- Benchmarks and optimizations
- Authorization
- Site visualization
- Creating crate
