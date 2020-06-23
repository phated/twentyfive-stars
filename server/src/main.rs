mod auth;
mod data;
mod database;
mod graphql_schema;
mod middleware;
mod request;
mod schema;
mod state;

use auth::{AuthClient, Bearer};
use database::Database;
use graphql_schema::{ContextData, MutationRoot, QueryRoot};
use state::State;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use dotenv::dotenv;

use std::env;
use tide::{
    http::{headers, mime},
    Redirect, Request, Response, Server, StatusCode,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn handle_graphql(req: Request<State>) -> tide::Result {
    let schema = req.state().schema.clone();
    async_graphql_tide::graphql(req, schema, |query_builder| query_builder).await
}

async fn handle_graphiql(req: Request<State>) -> tide::Result {
    let base_config = GraphQLPlaygroundConfig::new("/");

    let body = if let Some(bearer) = req.ext::<Bearer>() {
        let bearer_token = bearer.secret().clone();
        let config = base_config.with_header("Authorization", &bearer_token);
        playground_source(config)
    } else {
        playground_source(base_config)
    };

    let mut resp = Response::new(StatusCode::Ok);
    resp.insert_header(headers::CONTENT_TYPE, mime::HTML);

    resp.set_body(body);

    Ok(resp)
}

async fn handle_login(req: Request<State>) -> tide::Result {
    let State { ref auth, .. } = req.state();

    let (auth_url, state_cookie) = auth.get_login_redirect();

    let mut resp: Response = Redirect::new(auth_url.to_string()).into();

    // TODO: Secure cookies
    resp.insert_cookie(state_cookie.into());

    Ok(resp)
}

async fn handle_logout(req: Request<State>) -> tide::Result {
    let State { ref auth, .. } = req.state();

    let logout_url = auth.get_logout_redirect();

    let resp: Response = Redirect::new(logout_url.to_string()).into();

    // TODO: Clear a bearer cookie if I save it

    Ok(resp)
}

fn main() -> Result<()> {
    dotenv()?;
    env_logger::init();

    let database_url = env::var("DATABASE_URL")?;
    let listen_addr = env::var("LISTEN_ADDR").unwrap_or(String::from("0.0.0.0:3000"));

    let auth = envy::prefixed("OAUTH_").from_env::<AuthClient>()?;

    smol::block_on(async {
        println!("Playground: http://{}", listen_addr);

        let db = Database::new(&database_url).await?;

        // TODO: The Tide example says that it is probably worth making the
        // schema a singleton using lazy_static library
        let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
            .data(ContextData { db })
            .register_type::<schema::interfaces::Node>()
            .register_type::<schema::interfaces::Card>()
            .finish();

        // TODO: fix ? unwrapping
        let jwks = request::jwks().await.unwrap();

        let app_state = State { schema, auth, jwks };

        let mut app = Server::with_state(app_state);

        app.middleware(middleware::cors());

        app.at("/")
            .middleware(middleware::authenticate_bearer())
            .post(handle_graphql);
        // TODO: I don't actually like using the middleware here
        // It doesn't erase the code & state so I get an error on refresh
        app.at("/")
            .middleware(middleware::obtain_bearer())
            .get(handle_graphiql);
        app.at("/login").get(handle_login);
        app.at("/logout").get(handle_logout);

        app.listen(listen_addr).await?;

        Ok(())
    })
}
