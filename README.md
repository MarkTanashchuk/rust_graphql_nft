# Rust GraphQL Server for searching NFT (meta)data

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

Start server

```cli
cargo run --release
```

Open localhost:{PORT}. By default [localhost:8080](http://localhost:8080)

## Get Started

Note: Only Ethereum address format and ERC721 standard are supported

### Custom queries

The server is written using [async-graphql](https://async-graphql.github.io/async-graphql/en/quickstart.html), so to add new queries you need to update the graphql/mod.rs file according to docs.

### Custom Metadata

To add custom metadata formats with custom fields, add a new file with the required structure to the src/graphql/token/metadata folder.

### Example of getting single NFT

```graphql
# Variables
# {
#   "address": "0x7f371bed0bdb2012c01f219ca1c4cbcb35f37aef",
#   "id": 7
# }

query GetTokenWithMetadata($address: String!, $id: String!) {
  ethToken(address: $address, id: $id) {
    tokenAddress
    tokenId
    tokenURI
    tokenMetadata {
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

```graphql
# Variables
# {
#   "address": "0x7f371bed0bdb2012c01f219ca1c4cbcb35f37aef"
# }

query GetTokenWithMetadata($address: String!) {
  ethTokens(address: $address) {
    tokenAddress
    tokenId
    tokenURI
    tokenMetadata {
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

## Troubleshooting

### Api error: code 401

The `TRANSPORT_URL` in the `.env` file is set incorrectly or the provided `API_KEY` does not allow unauthorized access

In case of `infura`, create a new Web3 API, click Manage Key and copy the full Ethereum endpoint into `TRANSPORT_URL` in the `.env` file.

### Abi error: Invalid data

Parameters provided to the `QueryAPI::query` function may be invalid(<https://github.com/tomusdrw/rust-web3/issues/383>)

### Failed to fetch

This may be a server-side error. Check the error message in the terminal for more information.

## License

Licensed under MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
