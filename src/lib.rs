#![warn(missing_docs)]
//! Unofficial Auth0 Management API

use std::borrow::Borrow;
use std::error::Error;
use std::fmt::{Display, Formatter};

use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::export::fmt::Debug;
use serde::Deserialize;

#[doc(inline)]
pub use api::*;
pub use page::*;
pub use sort::*;
pub use users::*;

use crate::rate::{RateLimit, RateLimitError, RateLimitResponse};
use crate::token::{TokenError, TokenManager};

pub mod sort;

#[allow(missing_docs)]
pub mod api;
pub mod page;
#[doc(hidden)]
pub mod rate;
#[doc(hidden)]
pub mod token;
pub mod users;

/// Auth0 management client.
pub struct Auth0 {
  rate: RateLimit,
  token: TokenManager,
  client: Client,
}

impl Auth0 {
  /// Create Auth0 client
  pub fn builder() -> Auth0Builder {
    Default::default()
  }

  /// Query API
  pub async fn query<R>(&mut self, req: &R) -> Result<R::Response, Auth0Error>
  where
    R: Auth0Request + ?Sized,
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
      Ok(
        res
          .rate_limit(&mut self.rate)?
          .json::<R::Response>()
          .await?,
      )
    } else {
      Err(Auth0Error::from(res.json::<Auth0ErrorResponse>().await?))
    }
  }
}

/// Management client interface.
pub struct Auth0Builder {
  domain: Option<String>,
  audience: Option<String>,
  client_id: Option<String>,
  client_secret: Option<String>,
}

impl Auth0Builder {
  /// Get instance of management client builder.
  pub fn new() -> Self {
    Default::default()
  }

  /// Get instance of management client.
  ///
  /// Creates instance of management client and validates builder options.  Valid builder options
  /// requires all fields to be populated.
  pub fn build(self) -> Result<Auth0, Auth0BuilderError> {
    let client = Client::new();
    let domain = self.domain.ok_or(Auth0BuilderError::MissingDomain)?;
    let audience = self.audience.ok_or(Auth0BuilderError::MissingAudience)?;
    let client_id = self.client_id.ok_or(Auth0BuilderError::MissingClientID)?;
    let client_secret = self
      .client_secret
      .ok_or(Auth0BuilderError::MissingClientSecret)?;

    Ok(Auth0 {
      rate: RateLimit::new(),
      token: TokenManager::new(
        client.clone(),
        &domain,
        &audience,
        &client_id,
        &client_secret,
      ),
      client,
    })
  }

  /// The auth0 tenant domain.
  ///
  /// The domain can be found in the Auth0 dashboard under your application settings.  Domain
  /// will be the field labeled `Domain`.  Domains will be typically formatted as
  /// `example.[us|eu|au].auth0.com`.
  pub fn domain(mut self, domain: &str) -> Self {
    self.domain = Some(domain.to_owned());
    self
  }

  /// The value of the Identifier field of the `Auth0 Management API`.  
  ///
  /// The audience can be found in the Auth0 dashboard under you `API` settings.  Audience will be
  /// in the field labeled `Identifier`.  The default management API identifier will be formatted as
  /// `https://example.eu.auth0.com/api/v2`
  pub fn audience(mut self, audience: &str) -> Self {
    self.audience = Some(audience.to_owned());
    self
  }

  /// The value of the Client ID field of the Machine-to-Machine application.
  ///
  /// The client id can be found in the Auth0 dashboard under your application settings.  Client ID
  /// will be the field labeled `Client ID`.
  pub fn client_id(mut self, client_id: &str) -> Self {
    self.client_id = Some(client_id.to_owned());
    self
  }

  /// The value of the Client Secret field of the Machine-to-Machine application.
  ///
  /// The client secret can be found in the Auth0 dashboard under your application settings.  
  /// Client Secret will be the field labeled `Client Secret`.
  pub fn client_secret(mut self, client_secret: &str) -> Self {
    self.client_secret = Some(client_secret.to_owned());
    self
  }
}

impl Default for Auth0Builder {
  fn default() -> Self {
    Self {
      domain: None,
      audience: None,
      client_id: None,
      client_secret: None,
    }
  }
}

#[derive(Debug)]
pub enum Auth0Error {
  Http(reqwest::Error),
  Token(TokenError),
  Auth0(String),
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

#[derive(Deserialize)]
pub struct Auth0ErrorResponse {
  message: String,
}

impl From<Auth0ErrorResponse> for Auth0Error {
  fn from(inner: Auth0ErrorResponse) -> Self {
    Auth0Error::Auth0(inner.message)
  }
}

/// The error type which is returned from building a [Auth0].
#[derive(Debug, PartialOrd, PartialEq)]
pub enum Auth0BuilderError {
  /// Indicates builder didn't set [Auth0Builder::domain].
  MissingDomain,
  /// Indicates builder didn't set [Auth0Builder::audience].
  MissingAudience,
  /// Indicates builder didn't set [Auth0Builder::client_id].
  MissingClientID,
  /// Indicates builder didn't set [Auth0Builder::client_secret].
  MissingClientSecret,
}

impl Display for Auth0BuilderError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for Auth0BuilderError {}

pub trait Auth0Request {
  type Response: DeserializeOwned;

  fn build<F>(&self, factory: F) -> RequestBuilder
  where
    F: FnOnce(Method, &str) -> RequestBuilder;
}
