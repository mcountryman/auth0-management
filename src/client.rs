use std::error::Error;

use async_trait::async_trait;
use reqwest::{Method, RequestBuilder};
use serde::de::DeserializeOwned;

use crate::rate::RateLimit;
use crate::token::TokenManager;

pub struct Client {
  rate: RateLimit,
  token: TokenManager,
  client: reqwest::Client,
}

pub struct ClientRequest(RequestBuilder);

impl Client {
  pub fn get(
    &mut self,
    path: &str,
  ) -> Result<RequestBuilder, Box<dyn Error + Send + Sync>> {
    self.request(Method::GET, path)
  }

  pub fn put(
    &mut self,
    path: &str,
  ) -> Result<RequestBuilder, Box<dyn Error + Send + Sync>> {
    self.request(Method::PUT, path)
  }

  pub fn patch(
    &mut self,
    path: &str,
  ) -> Result<RequestBuilder, Box<dyn Error + Send + Sync>> {
    self.request(Method::PATCH, path)
  }

  pub fn post(
    &mut self,
    path: &str,
  ) -> Result<RequestBuilder, Box<dyn Error + Send + Sync>> {
    self.request(Method::POST, path)
  }

  pub fn delete(
    &mut self,
    path: &str,
  ) -> Result<RequestBuilder, Box<dyn Error + Send + Sync>> {
    self.request(Method::DELETE, path)
  }

  pub fn request(
    &mut self,
    method: Method,
    path: &str,
  ) -> Result<RequestBuilder, Box<dyn Error + Send + Sync>> {
    Ok(
      self
        .client
        .request(method, &format!("{}/{}", self.token.domain, path)),
    )
  }

  pub async fn send(
    &mut self,
    req: RequestBuilder,
  ) -> Result<(), Box<dyn Error + Send + Sync>> {
    let token = self.token.get_token().await?;
    let res = req
      .header("authentication", format!("Basic: {}", token))
      .send()
      .await?;

    self.rate.read(&res)?;

    Ok(())
  }

  pub async fn json<T: DeserializeOwned>(
    &mut self,
    req: RequestBuilder,
  ) -> Result<T, Box<dyn Error + Send + Sync>> {
    let token = self.token.get_token().await?;
    let res = req
      .header("authentication", format!("Basic: {}", token))
      .send()
      .await?;

    self.rate.read(&res)?;

    Ok(res.json::<T>().await?)
  }
}

#[async_trait]
pub trait ClientRequestBuilder {
  async fn send_pass(
    self,
    client: &mut Client,
  ) -> Result<(), Box<dyn Error + Send + Sync>>;
  async fn send_json<T: DeserializeOwned>(
    self,
    client: &mut Client,
  ) -> Result<T, Box<dyn Error + Send + Sync>>;
}

#[async_trait]
impl ClientRequestBuilder for RequestBuilder {
  async fn send_pass(
    self,
    client: &mut Client,
  ) -> Result<(), Box<dyn Error + Send + Sync>> {
    client.send(self).await
  }

  async fn send_json<T: DeserializeOwned>(
    self,
    client: &mut Client,
  ) -> Result<T, Box<dyn Error + Send + Sync>> {
    client.json(self).await
  }
}
