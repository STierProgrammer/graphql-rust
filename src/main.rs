use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::Router;
use axum::routing::post;
use database::db::DB;
use database::handlers::query_engine::Query;
mod database;

async fn graphql_handler(graph_qlrequest: GraphQLRequest)
 -> GraphQLResponse {
    let query = Query { db: DB };

    let schema = Schema::new(
        query,
        EmptyMutation,
        EmptySubscription
    );

    let res = schema.execute(graph_qlrequest
        .into_inner())
        .await;

    res.into()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/gql", post(graphql_handler));

    let listener = tokio
    ::net
    ::TcpListener
    ::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Listening....!");

    axum::serve(listener, app)
        .await
        .unwrap()
}