use oauth2::{AuthorizationCode, CsrfToken};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct OAuthQuerystring {
    pub state: CsrfToken,
    pub code: AuthorizationCode,
}
