use oauth2::CsrfToken;
use tide::http::cookies::{Cookie, SameSite};
use time::Duration;

#[derive(Debug, Clone)]
pub struct StateCookie {
    token: CsrfToken,
}

impl From<CsrfToken> for StateCookie {
    fn from(token: CsrfToken) -> Self {
        Self { token }
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
