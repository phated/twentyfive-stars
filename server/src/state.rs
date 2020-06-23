use crate::auth::{OAuthClient, JWKS};
use crate::graphql_schema::Schema;

pub struct State {
    pub schema: Schema,
    pub oauth_client: OAuthClient,
    // TODO: Figure out caching with refresh on the JWKS
    pub jwks: JWKS,
}
