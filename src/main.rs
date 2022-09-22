mod contract;
mod schema;

use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use schema::QueryRoot;

async fn index(
    schema: web::Data<Schema<QueryRoot, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn gql_playgound() -> HttpResponse {
    let port = std::env::var("PORT").expect("PORT should be set");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint(&format!("http://localhost:{port}"))
                .finish(),
        )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let port = std::env::var("PORT").expect("PORT should be set");
    println!("GraphiQL IDE: http://localhost:{}", port);

    // let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);
    // let query_1 = r#"query GetTokenWithMetadata($tokenAddress: String!, $tokenId: String!) {
    //     token(tokenAddress: $tokenAddress, tokenId: $tokenId) {
    //         tokenURI
    //     }
    // }"#;

    // let query_2 = r#"query GetTokenWithMetadata($tokenAddress: String!, $tokenId: String!) {
    //     token(tokenAddress: $tokenAddress, tokenId: $tokenId) {
    //         tokenURI
    //     }
    // }"#;

    // assert_ne!(schema.execute(query_1).await, schema.execute(query_2).await);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Schema::new(
                QueryRoot,
                EmptyMutation,
                EmptySubscription,
            )))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(gql_playgound))
    })
    .bind(format!("127.0.0.1:{port}"))?
    .run()
    .await
}
