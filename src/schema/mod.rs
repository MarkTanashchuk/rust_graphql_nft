mod token;
use async_graphql::*;
use std::str::FromStr;
use token::{Info, Metadata, Token, TokenMetadata};
use web3::types::U256;

use crate::contract::CustomContract;

struct EthAddress(String);
impl EthAddress {
    pub fn new(address: String) -> Result<Self, String> {
        let chars = address.chars().collect::<Vec<char>>();

        if let [first_char, second_char, rest @ ..] = &chars[..] {
            if first_char == &'0' && second_char == &'x' {
                return if rest.len() == 40 {
                    Ok(Self(address))
                } else {
                    Err("Invalid address length".to_string())
                };
            }
        }

        Err("Invalid address format".to_string())
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn token(&self, token_address: String, token_id: String) -> FieldResult<Token> {
        let (token_uri, token_id) = {
            let address = EthAddress::new(token_address.clone())?;
            let token_id = U256::from_str(&token_id)?;
            let contract = CustomContract::new(address.0);
            let token_uri = contract.query::<String, _>("tokenURI", token_id).await?;

            (token_uri, token_id)
        };

        if token_uri.starts_with("ipfs://") {
            return Err(async_graphql::Error::new("IPFS is not supported yet")
                .extend_with(|_, e| e.set("details", "CAN_NOT_PROCESS")));
        }

        let info = if token_uri.ends_with(".json") {
            reqwest::get(&token_uri).await?.json::<Info>().await?
        } else {
            return Err(
                async_graphql::Error::new("Not yet supported tokenURI format")
                    .extend_with(|_, e| e.set("details", "CAN_NOT_PROCESS")),
            );
        };

        let (height, width) = {
            let dimensions = info
                .media
                .dimensions
                .split("x")
                .map(|string| string.parse::<i32>().expect("Always valid numbers"))
                .collect::<Vec<i32>>();
            (dimensions[0], dimensions[1])
        };

        Ok(Token {
            token_uri: token_uri,
            token_id: token_id.to_string(),
            token_address,
            metadata: Some(TokenMetadata {
                name: info.name,
                description: info.description,
                external_url: info.image.clone(),
                attributes: info.attributes,
                image: Metadata {
                    url: info.image,
                    mime_type: info.media.mime_type,
                    height,
                    width,
                },
            }),
        })
    }
}
