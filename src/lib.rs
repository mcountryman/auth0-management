#![warn(missing_docs)]
//!
//!
//!
use std::error::Error;

use async_trait::async_trait;
use reqwest::{Client, Method, RequestBuilder, Response, Url};
use serde::{de::DeserializeOwned, Deserialize};

use crate::rate::{RateLimit, RateLimitError};
use crate::token::{TokenError, TokenManager};
use serde::export::fmt::Debug;
use std::fmt::{Display, Formatter};

#[allow(missing_docs)]
pub mod api;
#[doc(hidden)]
pub mod rate;
#[doc(hidden)]
pub mod token;

/// Auth0 management client.
pub struct ManagementClient {
  rate: RateLimit,
  token: TokenManager,
  client: Client,
}

impl ManagementClient {
  /// Get management client builder.
  pub fn builder() -> ManagementClientBuilder {
    ManagementClientBuilder::new()
  }

  pub(crate) fn get(
    &mut self,
    path: &str,
  ) -> Result<RequestBuilder, Box<dyn Error + Send + Sync>> {
    self.request(Method::GET, path)
  }

  // pub(crate) fn put(
  //   &mut self,
  //   path: &str,
  // ) -> Result<RequestBuilder, Box<dyn Error + Send + Sync>> {
  //   self.request(Method::PUT, path)
  // }

  pub(crate) fn patch(
    &mut self,
    path: &str,
  ) -> Result<RequestBuilder, Box<dyn Error + Send + Sync>> {
    self.request(Method::PATCH, path)
  }

  pub(crate) fn post(
    &mut self,
    path: &str,
  ) -> Result<RequestBuilder, Box<dyn Error + Send + Sync>> {
    self.request(Method::POST, path)
  }

  pub(crate) fn delete(
    &mut self,
    path: &str,
  ) -> Result<RequestBuilder, Box<dyn Error + Send + Sync>> {
    self.request(Method::DELETE, path)
  }

  pub(crate) fn request(
    &mut self,
    method: Method,
    path: &str,
  ) -> Result<RequestBuilder, Box<dyn Error + Send + Sync>> {
    Ok(self.client.request(
      method,
      Url::parse(&format!("https://{}", self.token.domain))?.join(path)?,
    ))
  }

  pub(crate) async fn send(
    &mut self,
    req: RequestBuilder,
  ) -> Result<Response, ManagementClientError> {
    let token = self.token.get_token().await?;
    let res = req
      .header("Authorization", format!("Bearer {}", token))
      .send()
      .await?;

    if !res.status().is_success() {
      return Err(ManagementClientError::from(
        res.json::<Auth0ErrorResponse>().await?,
      ));
    }

    self.rate.read(&res)?;

    Ok(res)
  }

  pub(crate) async fn json<T: DeserializeOwned>(
    &mut self,
    req: RequestBuilder,
  ) -> Result<T, Box<dyn Error + Send + Sync>> {
    Ok(self.send(req).await?.json::<T>().await?)
  }
}

/// Management client interface.
pub struct ManagementClientBuilder {
  domain: Option<String>,
  audience: Option<String>,
  client_id: Option<String>,
  client_secret: Option<String>,
}

impl ManagementClientBuilder {
  /// Get instance of management client builder.
  pub fn new() -> Self {
    Default::default()
  }

  /// Get instance of management client.
  ///
  /// Creates instance of management client and validates builder options.  Valid builder options
  /// requires all fields to be populated.
  pub fn build(self) -> Result<ManagementClient, ManagementClientBuilderError> {
    let client = Client::new();
    let domain = self
      .domain
      .ok_or(ManagementClientBuilderError::MissingDomain)?;
    let audience = self
      .audience
      .ok_or(ManagementClientBuilderError::MissingAudience)?;
    let client_id = self
      .client_id
      .ok_or(ManagementClientBuilderError::MissingClientID)?;
    let client_secret = self
      .client_secret
      .ok_or(ManagementClientBuilderError::MissingClientSecret)?;

    Ok(ManagementClient {
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

impl Default for ManagementClientBuilder {
  fn default() -> Self {
    Self {
      domain: None,
      audience: None,
      client_id: None,
      client_secret: None,
    }
  }
}

#[doc(hidden)]
#[async_trait]
pub trait ClientRequestBuilder {
  async fn send_pass(
    self,
    client: &mut ManagementClient,
  ) -> Result<(), Box<dyn Error + Send + Sync>>;
  async fn send_json<T: DeserializeOwned>(
    self,
    client: &mut ManagementClient,
  ) -> Result<T, Box<dyn Error + Send + Sync>>;
}

#[async_trait]
impl ClientRequestBuilder for RequestBuilder {
  async fn send_pass(
    self,
    client: &mut ManagementClient,
  ) -> Result<(), Box<dyn Error + Send + Sync>> {
    client.send(self).await?;
    Ok(())
  }

  async fn send_json<T: DeserializeOwned>(
    self,
    client: &mut ManagementClient,
  ) -> Result<T, Box<dyn Error + Send + Sync>> {
    client.json(self).await
  }
}

/// The error type which is returned from building a [ManagementClient].
#[derive(Debug, PartialOrd, PartialEq)]
pub enum ManagementClientBuilderError {
  /// Indicates builder didn't set [ManagementClientBuilder::domain].
  MissingDomain,
  /// Indicates builder didn't set [ManagementClientBuilder::audience].
  MissingAudience,
  /// Indicates builder didn't set [ManagementClientBuilder::client_id].
  MissingClientID,
  /// Indicates builder didn't set [ManagementClientBuilder::client_secret].
  MissingClientSecret,
}

impl Display for ManagementClientBuilderError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for ManagementClientBuilderError {}

/// The error type which is returned from performing a request through [ManagementClient].
#[derive(Debug)]
pub enum ManagementClientError {
  /// Auth0 error message response.
  Auth0(Auth0ErrorResponse),
  /// Auth0 authentication error.
  Auth0Token(TokenError),
  /// Generic HTTP transport error.
  Transport(reqwest::Error),
  /// Generic HTTP response malformed error.
  ///
  /// Can occur when Auth0 does not provide `x-rate-limit` headers.  Typically when an error occurs
  /// on Auth0's end.
  MalformedResponse(RateLimitError),
}

impl Display for ManagementClientError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self)
  }
}

impl Error for ManagementClientError {}

/// The response definition which is returned when Auth0 responds with a non-success response.
#[derive(Clone, PartialOrd, PartialEq, Deserialize)]
pub struct Auth0ErrorResponse {
  message: String,
}

impl Debug for Auth0ErrorResponse {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl From<TokenError> for ManagementClientError {
  fn from(err: TokenError) -> Self {
    ManagementClientError::Auth0Token(err)
  }
}

impl From<Auth0ErrorResponse> for ManagementClientError {
  fn from(res: Auth0ErrorResponse) -> Self {
    ManagementClientError::Auth0(res)
  }
}

impl From<reqwest::Error> for ManagementClientError {
  fn from(err: reqwest::Error) -> Self {
    ManagementClientError::Transport(err)
  }
}

impl From<RateLimitError> for ManagementClientError {
  fn from(err: RateLimitError) -> Self {
    ManagementClientError::MalformedResponse(err)
  }
}
