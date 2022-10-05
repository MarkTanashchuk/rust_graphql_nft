pub mod gql;

use super::QueryRoot;
use actix_web::{guard, web, web::Data, App, HttpServer};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use gql::{index, playgound};

#[actix_web::main]
pub async fn start_server(port: String) -> std::io::Result<()> {
    println!("GraphiQL IDE: http://localhost:{port}");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Schema::new(
                QueryRoot,
                EmptyMutation,
                EmptySubscription,
            )))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(playgound))
    })
    .bind(format!("127.0.0.1:{port}"))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn valid_query() {
        let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);

        let empty_query = "{}";
        let single_field = "query GetTokenWithMetadata($tokenAddress: String!, $tokenId: String!) {
            token(tokenAddress: $tokenAddress, tokenId: $tokenId) {
                tokenURI
            }
        }";
        let full_schema = "query GetTokenWithMetadata($tokenAddress: String!, $tokenId: String!) {
            token(tokenAddress: $tokenAddress, tokenId: $tokenId) {
                tokenAddress
                tokenId
                tokenURI
                metadata {
                    name
                    description
                    image
                    external_url
                    data
                    media
                    attributes {
                        traitType
                        value
                    }
                }
            }
        }";

        let queries = vec![empty_query, single_field, full_schema];

        for query in queries {
            let executed_query = schema.execute(query).await.into_result();

            assert!(executed_query.is_ok());
        }
    }

    #[tokio::test]
    async fn invalid_query() {
        let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);

        let empty_string = "";
        let invalid_fields = "{ something }";

        let queries = vec![empty_string, invalid_fields];

        for query in queries {
            let executed_query = schema.execute(query).await.into_result();

            assert!(executed_query.is_err());
        }
    }
}
