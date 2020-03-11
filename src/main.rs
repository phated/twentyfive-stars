#[macro_use]
extern crate diesel;

use iron::prelude::*;
use juniper_iron::{GraphQLHandler, GraphiQLHandler};
use logger::Logger;
use mount::Mount;

mod data;
mod database;
mod database_schema;
mod graphql_schema;
mod pagination;
mod schema;

use database::establish_connection;
use graphql_schema::{Context, Mutation, Query};

fn context_factory(_: &mut Request) -> IronResult<Context> {
    Ok(Context {
        connection: establish_connection(),
    })
}

fn main() {
    env_logger::init();

    let mut mount = Mount::new();

    let graphql_endpoint = GraphQLHandler::new(context_factory, Query, Mutation);
    let graphiql_endpoint = GraphiQLHandler::new("/graphql");

    mount.mount("/", graphiql_endpoint);
    mount.mount("/graphql", graphql_endpoint);

    let (logger_before, logger_after) = Logger::new(None);

    let mut chain = Chain::new(mount);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    // let host = env::var("LISTEN").unwrap_or_else(|_| "0.0.0.0:8080".to_owned());
    // println!("GraphQL server started on {}", host);

    let _server = Iron::new(chain).http("localhost:3000").unwrap();
    println!("On 3000");
}
