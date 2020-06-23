use biscuit::jwk::JWKSet;
use biscuit::Empty;
use failure::Fail;
use http_client::h1::H1Client;
use http_client::HttpClient;
use tide::http::{Method, Request, Response, Url};

// TODO: I really hate this, need to drop `failure`
#[derive(Debug, Fail)]
#[fail(display = "{}", _0)]
pub struct Error(String);

pub async fn client(request: Request) -> Result<Response, Error> {
    let client = H1Client::new();
    match client.send(request).await {
        Ok(response) => Ok(response),
        Err(_) => Err(Error("Something broke".into())),
    }
}

pub async fn jwks() -> Result<JWKSet<Empty>, Error> {
    let client = H1Client::new();
    let request = Request::new(
        Method::Get,
        Url::parse("https://twentyfive-stars.us.auth0.com/.well-known/jwks.json").unwrap(),
    );
    match client.send(request).await {
        Ok(mut response) => match response.body_json::<JWKSet<Empty>>().await {
            Ok(jwks) => Ok(jwks),
            Err(_err) => Err(Error("Unable to parse JWKS".into())),
        },
        Err(_err) => Err(Error("Unable to get JWKS".into())),
    }
}
