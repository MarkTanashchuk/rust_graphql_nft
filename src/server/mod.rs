/// Contains the functions for `GraphiQL` playground and `GraphQL` schema creation
pub mod gql;

use super::QueryRoot;
use actix_web::{guard, web, web::Data, App, HttpServer};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use gql::{index, playgound};

/// Starting point of the application where the server is initialized
#[actix_web::main]
pub async fn start_server(port: String) -> std::io::Result<()> {
    println!("GraphiQL IDE: http://localhost:{port}");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Schema::new(
                QueryRoot::default(),
                EmptyMutation,
                EmptySubscription,
            )))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(playgound))
    })
    .bind(format!("127.0.0.1:{port}"))
    .expect(&(port + " port cannot be used"))
    .run()
    .await
}
