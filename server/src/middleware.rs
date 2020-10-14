use crate::auth::{BearerToken, OAuthQuerystring};
use crate::state::State;

use oauth2::{CsrfToken, TokenResponse};
use std::sync::Arc;
use tide::security::CorsMiddleware;
use tide::{Middleware, Next, Request, Response, Result, StatusCode};
use tide_http_auth::{Authentication, BearerAuthScheme};

pub fn cors() -> CorsMiddleware {
    // TODO: Constrain this
    CorsMiddleware::new()
}

pub fn authenticate_bearer<T: Send + Sync + 'static>() -> Authentication<T, BearerAuthScheme> {
    Authentication::new(BearerAuthScheme::default())
}

pub fn obtain_bearer() -> ObtainBearer {
    ObtainBearer::new()
}

// TODO: Switch to function middleware
#[derive(Debug, Clone)]
pub struct ObtainBearer;

impl ObtainBearer {
    pub fn new() -> Self {
        Self
    }
}
// TODO: Should this even be middleware?
#[async_trait::async_trait]
impl Middleware<Arc<State>> for ObtainBearer {
    async fn handle(&self, mut req: Request<Arc<State>>, next: Next<'_, Arc<State>>) -> Result {
        let (state, code) = match req.query() {
            Ok(OAuthQuerystring { state, code }) => (state, code),
            Err(_err) => {
                return Err(tide::Error::from_str(
                    StatusCode::BadRequest,
                    "Invalid parameters",
                ))
            }
        };

        let auth = &req.state().auth;
        let state_cookie = req.cookie("state");

        let expected_state = if let Some(cookie) = state_cookie.clone() {
            CsrfToken::new(cookie.value().into())
        } else {
            return Err(tide::Error::from_str(
                StatusCode::BadRequest,
                "Invalid session",
            ));
        };

        // TODO: Constant time comparison?
        if state != expected_state {
            return Err(tide::Error::from_str(
                StatusCode::BadRequest,
                "Invalid session",
            ));
        }

        let token = match auth.exchange_code(code).await {
            Ok(token) => token.access_token().clone(),
            Err(_err) => {
                return Err(tide::Error::from_str(
                    StatusCode::Unauthorized,
                    "Unable to authenticate",
                ));
            }
        };

        let bearer: BearerToken = token.into();
        req.set_ext(bearer);

        let mut resp: Response = next.run(req).await;

        // Cleanup the state cookie
        if let Some(cookie) = state_cookie.clone() {
            resp.remove_cookie(cookie);
        }

        Ok(resp)
    }
}
