//! Auth0 request client.
use std::error::Error;
use std::fmt::{Display, Formatter};

use reqwest::Client;
use serde::export::fmt::Debug;
use serde::Deserialize;

use crate::rate::{RateLimit, RateLimitError, RateLimitResponse};
use crate::token::{TokenError, TokenManager};
use crate::Auth0RequestBuilder;

/// Auth0 management client.
pub struct Auth0Client {
  pub(crate) rate: RateLimit,
  pub(crate) token: TokenManager,
  pub(crate) client: Client,
}

impl Auth0Client {
  /// Query API
  pub async fn query<R>(&self, req: &R) -> Result<R::Response, Auth0Error>
  where
    R: Auth0RequestBuilder + ?Sized,
  {
    let token = self.token.get_token().await?;
    let res = req
      .build(|method, path| {
        self
          .client
          .request(method, &format!("https://{}/{}", self.token.domain, path))
      })
      .bearer_auth(&token)
      .send()
      .await?;

    if res.status().is_success() {
      Ok(res.rate_limit(&self.rate)?.json::<R::Response>().await?)
    } else {
      Err(Auth0Error::from(res.json::<Auth0ErrorResponse>().await?))
    }
  }
}

/// The error returned when querying Auth0.
#[derive(Debug)]
pub enum Auth0Error {
  /// Generic http error.
  Http(reqwest::Error),
  /// Authentication token error.
  Token(TokenError),
  /// Auth0 server side error.
  Auth0(String),
  /// Auth0 rate limit error.
  RateLimit(RateLimitError),
}

impl Display for Auth0Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for Auth0Error {}

impl From<TokenError> for Auth0Error {
  fn from(inner: TokenError) -> Self {
    Auth0Error::Token(inner)
  }
}

impl From<reqwest::Error> for Auth0Error {
  fn from(inner: reqwest::Error) -> Self {
    Auth0Error::Http(inner)
  }
}

impl From<RateLimitError> for Auth0Error {
  fn from(inner: RateLimitError) -> Self {
    Auth0Error::RateLimit(inner)
  }
}

/// Auth0 error response.
#[derive(Deserialize)]
pub struct Auth0ErrorResponse {
  message: String,
}

impl From<Auth0ErrorResponse> for Auth0Error {
  fn from(inner: Auth0ErrorResponse) -> Self {
    Auth0Error::Auth0(inner.message)
  }
}
