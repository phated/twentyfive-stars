use oauth2::AccessToken;
use tide::http::cookies::{Cookie, SameSite};
use time::Duration;

#[derive(Debug, Clone)]
pub struct BearerToken {
    token: Option<AccessToken>,
}

impl BearerToken {
    pub fn empty() -> Self {
        Self { token: None }
    }
}

impl BearerToken {
    pub fn secret(&self) -> String {
        match self.token {
            Some(ref token) => format!("Bearer {}", token.secret()),
            None => "".into(),
        }
    }
}

impl From<AccessToken> for BearerToken {
    fn from(token: AccessToken) -> Self {
        Self { token: Some(token) }
    }
}

impl<'a> From<Cookie<'a>> for BearerToken {
    fn from(cookie: Cookie<'a>) -> Self {
        match cookie.value().into() {
            "" => Self::empty(),
            token => Self {
                token: Some(AccessToken::new(token.into())),
            },
        }
    }
}

impl<'a> Into<Cookie<'a>> for BearerToken {
    fn into(self) -> Cookie<'a> {
        let value = match self.token {
            Some(token) => token.secret().clone(),
            None => "".into(),
        };

        // TODO: Secure cookies
        Cookie::build("bearer", value)
            .path("/")
            .http_only(true)
            // TODO: This should be used in production when HTTPS only
            // .secure(true)
            .max_age(Duration::minutes(5))
            .same_site(SameSite::Lax)
            .finish()
    }
}
