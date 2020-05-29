mod data;
mod database;
mod graphql_schema;
mod schema;

use database::Database;
use graphql_schema::{ContextData, QueryRoot};

use async_graphql::http::playground_source;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use dotenv::dotenv;
use std::env;
use tide::{
    http::{headers, mime},
    security::CorsMiddleware,
    Request, Response, Server, StatusCode,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

struct AppState {
    schema: Schema<QueryRoot, EmptyMutation, EmptySubscription>,
}

async fn handle_graphql(cx: Request<AppState>) -> tide::Result {
    let schema = cx.state().schema.clone();
    async_graphql_tide::graphql(cx, schema, |query_builder| query_builder).await
}

async fn handle_graphiql(_: Request<AppState>) -> tide::Result {
    let resp = Response::new(StatusCode::Ok)
        .body_string(playground_source("/", None))
        .set_header(headers::CONTENT_TYPE, mime::HTML.to_string());

    Ok(resp)
}

fn main() -> Result<()> {
    dotenv()?;
    env_logger::init();

    let database_url = env::var("DATABASE_URL")?;
    let listen_addr = env::var("LISTEN_ADDR").unwrap_or(String::from("0.0.0.0:3000"));

    smol::block_on(async {
        println!("Playground: http://{}", listen_addr);

        let db = Database::new(&database_url).await?;

        // TODO: The Tide example says that it is probably worth making the
        // schema a singleton using lazy_static library
        let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
            .data(ContextData { db })
            .register_type::<schema::interfaces::Node>()
            .register_type::<schema::interfaces::Card>()
            .finish();

        let app_state = AppState { schema };

        let mut app = Server::with_state(app_state);

        // TODO: Constrain this
        let cors = CorsMiddleware::new();

        app.middleware(cors);

        app.at("/").post(handle_graphql);
        app.at("/").get(handle_graphiql);

        app.listen(listen_addr).await?;

        Ok(())
    })
}
