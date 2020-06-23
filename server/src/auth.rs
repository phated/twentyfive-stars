use crate::state::State;

use biscuit::jwa::SignatureAlgorithm;
use biscuit::jwk::JWKSet;
use biscuit::{Empty, JWT};
use oauth2::basic::BasicClient;
use oauth2::{AccessToken, AuthorizationCode, CsrfToken};
use serde::{Deserialize, Serialize};
use tide_http_auth::{BearerAuthRequest, Storage};

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
pub type OAuthClient = BasicClient;

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
