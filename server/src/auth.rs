use crate::request;
use crate::state::State;

use biscuit::jwa::SignatureAlgorithm;
use biscuit::jwk::JWKSet;
use biscuit::{Empty, JWT};
use oauth2::basic::{BasicClient, BasicRequestTokenError, BasicTokenResponse};
use oauth2::{
    url::Url, AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    RedirectUrl, TokenUrl,
};
use serde::{Deserialize, Serialize};
use tide::http::cookies::{Cookie, SameSite};
use tide_http_auth::{BearerAuthRequest, Storage};
use time::Duration;

#[derive(Deserialize, Debug, Clone)]
pub struct AuthClient {
    client_id: ClientId,
    client_secret: ClientSecret,
    base_url: Url,
    audience: String,
    login_redirect_url: RedirectUrl,
    logout_return_url: Url,
}

pub struct StateCookie {
    token: CsrfToken,
}

impl From<CsrfToken> for StateCookie {
    fn from(token: CsrfToken) -> Self {
        StateCookie { token }
    }
}

impl<'a> Into<Cookie<'a>> for StateCookie {
    fn into(self) -> Cookie<'a> {
        // TODO: Secure cookies
        Cookie::build("state", self.token.secret().clone())
            .path("/")
            .http_only(true)
            // TODO: This should be used in production when HTTPS only
            // .secure(true)
            .max_age(Duration::minutes(5))
            .same_site(SameSite::Lax)
            .finish()
    }
}

impl AuthClient {
    fn auth_url(&self) -> AuthUrl {
        let mut url = self.base_url.clone();
        url.set_path("/authorize");
        AuthUrl::from_url(url)
    }

    fn token_url(&self) -> TokenUrl {
        let mut url = self.base_url.clone();
        url.set_path("/oauth/token");
        TokenUrl::from_url(url)
    }

    fn logout_url(&self) -> Url {
        let client_id = self.client_id.clone();
        let return_to = self.logout_return_url.clone();
        let qs = format!(
            "client_id={}&returnTo={}",
            client_id.to_string(),
            return_to.to_string()
        );

        let mut url = self.base_url.clone();
        url.set_path("/v2/logout");
        url.set_query(Some(&qs));
        url
    }

    fn client(&self) -> BasicClient {
        let client_id = self.client_id.clone();
        let client_secret = Some(self.client_secret.clone());
        let auth_url = self.auth_url();
        let token_url = Some(self.token_url());
        let redirect_url = self.login_redirect_url.clone();

        let oauth_client = BasicClient::new(client_id, client_secret, auth_url, token_url)
            .set_redirect_url(redirect_url);

        oauth_client
    }

    pub fn get_login_redirect(&self) -> (Url, StateCookie) {
        let client = self.client().clone();
        let audience = self.audience.clone();

        // Generate the authorization URL to which we'll redirect the user.
        let (authorize_url, csrf_state) = client
            .authorize_url(CsrfToken::new_random)
            .add_extra_param("audience", &audience)
            .url();

        (authorize_url, csrf_state.into())
    }

    pub fn get_logout_redirect(&self) -> Url {
        self.logout_url()
    }

    pub async fn exchange_code(
        &self,
        code: AuthorizationCode,
    ) -> Result<BasicTokenResponse, BasicRequestTokenError<request::Error>> {
        let client = self.client().clone();
        client.exchange_code(code).request(request::client).await
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    permissions: Vec<String>,
}

impl User {
    pub fn empty() -> Self {
        User {
            permissions: Vec::with_capacity(0),
        }
    }
}

#[derive(Deserialize)]
pub struct OAuthQuerystring {
    pub state: CsrfToken,
    pub code: AuthorizationCode,
}

pub struct Bearer {
    token: AccessToken,
}

impl Bearer {
    pub fn secret(&self) -> String {
        format!("Bearer {}", self.token.secret())
    }
}

impl From<AccessToken> for Bearer {
    fn from(token: AccessToken) -> Self {
        Bearer { token }
    }
}

pub type JWKS = JWKSet<Empty>;

#[async_trait::async_trait]
impl Storage<User, BearerAuthRequest> for State {
    async fn get_user(&self, request: BearerAuthRequest) -> tide::Result<Option<User>> {
        let jwt: JWT<User, Empty> = JWT::new_encoded(&request.token);

        let user = match jwt.decode_with_jwks(&self.jwks, Some(SignatureAlgorithm::RS256)) {
            // TODO: Validate the temporal claims
            Ok(jwt) => {
                let claims = jwt.payload().unwrap();
                let user = claims.private.clone();
                user
            }
            Err(_err) => User::empty(),
        };

        Ok(Some(user))
    }
}
