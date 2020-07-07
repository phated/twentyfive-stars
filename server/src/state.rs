use crate::auth::{AuthClient, JWKS};
use crate::graphql_schema::Schema;

pub struct State {
    pub schema: Schema,
    pub auth: AuthClient,
    // TODO: Figure out caching with refresh on the JWKS
    pub jwks: JWKS,
}
