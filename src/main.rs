pub mod schema;
pub mod model;

use schema::{Query};

use async_graphql::{EmptyMutation, EmptySubscription, Schema, http::GraphiQLSource};
use async_graphql_axum::GraphQL;
use axum::{
    Router,
    response::{self, IntoResponse},
    routing::get
};
// use starwars::{QueryRoot, StarWars};
use tokio::net::TcpListener;
use reqwest::Client;


async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(client)
        .finish();
    let app = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));
    println!("GraphiQL IDE: http://localhost:8000");
    axum::serve(TcpListener::bind("127.0.0.1:8000").await.unwrap(), app)
        .await
        .unwrap();
}
