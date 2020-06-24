mod auth;
mod data;
mod database;
mod graphql_schema;
mod middleware;
mod request;
mod schema;
mod state;
mod user;

use auth::{AuthClient, BearerToken};
use database::Database;
use graphql_schema::{ContextData, MutationRoot, QueryRoot};
use state::State;
use user::User;

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
    let maybe_user = if let Some(user) = req.ext::<User>() {
        Some(user.clone())
    } else {
        None
    };

    async_graphql_tide::graphql(req, schema, |query_builder| {
        if let Some(user) = maybe_user.clone() {
            query_builder.data(user)
        } else {
            query_builder
        }
    })
    .await
}

async fn handle_graphiql(req: Request<State>) -> tide::Result {
    let bearer_token = if let Some(bearer_cookie) = req.cookie("bearer") {
        bearer_cookie.into()
    } else {
        BearerToken::empty()
    };

    let bearer_header = bearer_token.secret();

    let playground_config =
        GraphQLPlaygroundConfig::new("/").with_header("Authorization", &bearer_header);

    let body = playground_source(playground_config);

    let mut resp = Response::new(StatusCode::Ok);
    resp.insert_header(headers::CONTENT_TYPE, mime::HTML);

    // TODO: Should the bearer cookie be removed once we set up this Graphiql session?

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

    let mut resp: Response = Redirect::new(logout_url.to_string()).into();

    // Remove any bearer cookies we have hanging around
    if let Some(cookie) = req.cookie("bearer") {
        resp.remove_cookie(cookie);
    }

    Ok(resp)
}

async fn handle_login_success(req: Request<State>) -> tide::Result {
    if let Some(bearer) = req.ext::<BearerToken>() {
        let bearer_token = bearer.clone();

        let mut resp: Response = Redirect::new("/").into();

        // TODO: Secure cookies
        resp.insert_cookie(bearer_token.into());

        Ok(resp)
    } else {
        Err(tide::Error::from_str(
            StatusCode::BadRequest,
            "Unable to obtain auth token",
        ))
    }
}

fn main() -> Result<()> {
    dotenv()?;
    env_logger::init();

    let database_url = env::var("DATABASE_URL")?;
    let listen_addr = env::var("LISTEN_ADDR").unwrap_or(String::from("0.0.0.0:3000"));

    let auth = envy::prefixed("AUTH0_").from_env::<AuthClient>()?;

    smol::run(async {
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
        let jwks = request::jwks(auth.get_jwks_url()).await.unwrap();

        let app_state = State { schema, auth, jwks };

        let mut app = Server::with_state(app_state);

        app.middleware(middleware::cors());

        app.at("/")
            .middleware(middleware::authenticate_bearer())
            .post(handle_graphql);
        app.at("/").get(handle_graphiql);
        app.at("/login").get(handle_login);
        app.at("/login_success")
            .middleware(middleware::obtain_bearer())
            .get(handle_login_success);
        app.at("/logout").get(handle_logout);

        app.listen(listen_addr).await?;

        Ok(())
    })
}
