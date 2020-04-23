#[macro_use]
extern crate diesel;

mod data;
mod database;
mod database_schema;
mod graphql_schema;
mod pagination;
mod schema;

use graphql_schema::QueryRoot;

use async_graphql::http::{playground_source, GQLResponse};
use async_graphql::{EmptyMutation, EmptySubscription, QueryBuilder, Schema};
use async_graphql_warp::BadRequest;
use std::convert::Infallible;
use std::error::Error;
use warp::{http::Response, Filter, Rejection, Reply};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let pool = database::pool()?;

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(pool)
        .register_type::<data::Node>()
        .finish();

    // TODO: restrict this some
    let cors = warp::cors().allow_any_origin().allow_method("POST");

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, builder): (_, QueryBuilder)| async move {
            let resp = builder.execute(&schema).await;
            Ok::<_, Infallible>(warp::reply::json(&GQLResponse(resp)).into_response())
        },
    );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(playground_source("/", None))
    });

    let routes = graphql_post
        .with(cors)
        .or(graphql_playground)
        .with(warp::log("requests"))
        .recover(|err: Rejection| async move {
            if let Some(BadRequest(err)) = err.find() {
                return Ok::<_, Infallible>(warp::reply::with_status(
                    err.to_string(),
                    warp::http::StatusCode::BAD_REQUEST,
                ));
            }

            Ok(warp::reply::with_status(
                "INTERNAL_SERVER_ERROR".to_string(),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        });

    println!("Playground at: http://localhost:3000");

    warp::serve(routes).run(([0, 0, 0, 0], 3000)).await;

    Ok(())
}
