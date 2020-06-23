mod auth;
mod data;
mod database;
mod graphql_schema;
mod middleware;
mod request;
mod schema;
mod state;

use auth::Bearer;
use database::Database;
use graphql_schema::{ContextData, MutationRoot, QueryRoot};
use state::State;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use dotenv::dotenv;

use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, TokenUrl};
use std::env;
use tide::{
    http::{cookies, headers, mime},
    Redirect, Request, Response, Server, StatusCode,
};
use time::Duration;

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
    resp.insert_header(headers::CONTENT_TYPE, mime::HTML.to_string());

    resp.set_body(body);

    Ok(resp)
}

async fn handle_login(cx: Request<State>) -> tide::Result {
    let client = &cx.state().oauth_client;

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_extra_param("audience", "https://testing.twentyfivestars.com")
        .url();

    let state = csrf_state.secret().clone();

    let mut res: Response = Redirect::new(authorize_url.to_string()).into();

    // TODO: Secure cookies
    res.insert_cookie(
        cookies::Cookie::build("state", state)
            .path("/")
            .http_only(true)
            // TODO: This should be used in production when HTTPS only
            // .secure(true)
            .max_age(Duration::minutes(5))
            .same_site(cookies::SameSite::Lax)
            .finish(),
    );

    Ok(res)
}

fn main() -> Result<()> {
    dotenv()?;
    env_logger::init();

    let database_url = env::var("DATABASE_URL")?;
    let listen_addr = env::var("LISTEN_ADDR").unwrap_or(String::from("0.0.0.0:3000"));

    let oauth_client_id = ClientId::new(env::var("OAUTH_CLIENT_ID")?);
    let oauth_client_secret = ClientSecret::new(env::var("OAUTH_CLIENT_SECRET")?);
    let auth_url = AuthUrl::new(format!("https://{}/authorize", env::var("OAUTH_DOMAIN")?))?;
    let token_url = TokenUrl::new(format!("https://{}/oauth/token", env::var("OAUTH_DOMAIN")?))?;

    // Set up the config for the OAuth2 process.
    let oauth_client = BasicClient::new(
        oauth_client_id,
        Some(oauth_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_url(RedirectUrl::new(env::var("OAUTH_REDIRECT_URL")?)?);

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

        let app_state = State {
            schema,
            oauth_client,
            jwks,
        };

        let mut app = Server::with_state(app_state);

        app.middleware(middleware::cors());

        app.at("/")
            .middleware(middleware::authenticate_bearer())
            .post(handle_graphql);
        app.at("/")
            .middleware(middleware::obtain_bearer())
            .get(handle_graphiql);
        app.at("/login").get(handle_login);

        app.listen(listen_addr).await?;

        Ok(())
    })
}
