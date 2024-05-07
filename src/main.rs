mod db;
mod query_engine;
mod user_service;

use async_graphql::{EmptyMutation, EmptySubscription, Response};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use async_graphql::Schema;
use axum::Router;
use axum::routing::post;
use crate::query_engine::Query;
use crate::db::DB;

async fn graphql_handler(graph_qlrequest: GraphQLRequest) -> GraphQLResponse {
    let query = Query { db:DB };

    let schema = Schema::new(
        query,
        EmptyMutation,
        EmptySubscription
    );

    let response:Response = schema.execute(graph_qlrequest.into_inner()).await;

    response.into()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/gql", post(graphql_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening...!");
    axum::serve(listener, app).await.unwrap()
}
