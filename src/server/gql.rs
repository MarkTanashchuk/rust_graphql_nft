use crate::graphql::QueryRoot;
use actix_web::{web, HttpResponse};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

type RootSchema = web::Data<Schema<QueryRoot, EmptyMutation, EmptySubscription>>;

/// GraphQL endpoint
pub async fn index(schema: RootSchema, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

/// GraphiQL playground
pub async fn playgound() -> HttpResponse {
    let port = std::env::var("PORT").unwrap_or_else(|_| 8080.to_string());

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint(&format!("http://localhost:{port}"))
                .finish(),
        )
}
