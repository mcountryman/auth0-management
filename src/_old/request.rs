use async_trait::async_trait;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

use crate::{Auth0Client, Auth0Result};

/// Request
#[async_trait]
pub trait Auth0Request {
  /// Send request
  async fn send<T>(&self) -> Auth0Result<T>
  where
    T: DeserializeOwned + Send + Sync;
}

/// Simple request
#[async_trait]
pub trait Auth0RequestSimple {
  /// Send request to client
  async fn send_to<T>(&self, client: &Auth0Client) -> Auth0Result<T>
  where
    T: DeserializeOwned + Send + Sync;
}

/// Request builder
pub trait Auth0RequestBuilder {
  /// Build request
  fn build(&self, client: &Auth0Client) -> RequestBuilder;
}

#[async_trait]
impl<A: Auth0RequestBuilder + Send + Sync> Auth0RequestSimple for A {
  async fn send_to<T>(&self, client: &Auth0Client) -> Auth0Result<T>
  where
    T: DeserializeOwned + Send + Sync,
  {
    client.send(self.build(&client)).await
  }
}

#[async_trait]
impl<A: Auth0RequestBuilder + AsRef<Auth0Client> + Sync + Send> Auth0Request for A {
  async fn send<T>(&self) -> Auth0Result<T>
  where
    T: DeserializeOwned + Send + Sync,
  {
    let client = self.as_ref();
    let req = self.build(&client);

    client.send(req).await
  }
}
