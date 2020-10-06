use std::error::Error;

use async_trait::async_trait;
use reqwest::{Client, Method, RequestBuilder, Response, StatusCode, Url};
use serde::{de::DeserializeOwned, Deserialize};

use crate::rate::{RateLimit, RateLimitError};
use crate::token::{TokenError, TokenManager};
use serde::export::fmt::Debug;
use std::fmt::{Display, Formatter};

pub mod api;
pub mod rate;
pub mod token;

pub struct ManagementClient {
  rate: RateLimit,
  token: TokenManager,
  client: Client,
}

impl ManagementClient {
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
        res.json::<ErrorResponse>().await?,
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

pub struct ManagementClientBuilder {
  domain: Option<String>,
  audience: Option<String>,
  client_id: Option<String>,
  client_secret: Option<String>,
}

impl ManagementClientBuilder {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn build(self) -> Result<ManagementClient, BuilderError> {
    let client = Client::new();
    let domain = self.domain.ok_or(BuilderError::MissingDomain)?;
    let audience = self.audience.ok_or(BuilderError::MissingAudience)?;
    let client_id = self.client_id.ok_or(BuilderError::MissingClientID)?;
    let client_secret = self
      .client_secret
      .ok_or(BuilderError::MissingClientSecret)?;

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

  pub fn domain(mut self, domain: &str) -> Self {
    self.domain = Some(domain.to_owned());
    self
  }

  pub fn audience(mut self, audience: &str) -> Self {
    self.audience = Some(audience.to_owned());
    self
  }

  pub fn client_id(mut self, client_id: &str) -> Self {
    self.client_id = Some(client_id.to_owned());
    self
  }

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

impl Display for BuilderError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum BuilderError {
  MissingDomain,
  MissingAudience,
  MissingClientID,
  MissingClientSecret,
}

impl Error for BuilderError {}

#[derive(Debug)]
pub enum ManagementClientError {
  Auth0(ErrorResponse),
  Auth0Token(TokenError),
  Transport(reqwest::Error),
  MalformedResponse(RateLimitError),
}

impl Display for ManagementClientError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self)
  }
}

impl Error for ManagementClientError {}

#[derive(Clone, PartialOrd, PartialEq, Deserialize)]
pub struct ErrorResponse {
  message: String,
}

impl Debug for ErrorResponse {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl From<TokenError> for ManagementClientError {
  fn from(err: TokenError) -> Self {
    ManagementClientError::Auth0Token(err)
  }
}

impl From<ErrorResponse> for ManagementClientError {
  fn from(res: ErrorResponse) -> Self {
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
