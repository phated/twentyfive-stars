mod bearer_token;
mod client;
mod oauth_querystring;
mod permission;
mod state_cookie;

use crate::state::State;
use crate::user::User;

use biscuit::jwa::SignatureAlgorithm;
use biscuit::jwk::JWKSet;
use biscuit::{Empty, JWT};
use tide_http_auth::{BearerAuthRequest, Storage};

pub use bearer_token::BearerToken;
pub use client::AuthClient;
pub use oauth_querystring::OAuthQuerystring;
pub use permission::{Permission, PermissionGuard};
pub use state_cookie::StateCookie;

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
