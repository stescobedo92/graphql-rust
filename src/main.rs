mod db;
mod query_engine;
mod user_service;

use async_graphql::{EmptyMutation, EmptySubscription};
use async_graphql::parser::types::DirectiveLocation::Schema;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use crate::query_engine::Query;
use crate::db::DB;

async fn graphql_handler(graph_qlrequest: GraphQLRequest) -> GraphQLResponse {
    let query = Query { db:DB };

    let schema = Schema::new(
        query,
        EmptyMutation,
        EmptySubscription
    );

    let response = schema.execute(graph_qlrequest.into_inner()).await;

    response.into();
}

fn main() {
    println!("Hello, world!");
}
