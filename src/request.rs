use async_trait::async_trait;
use reqwest::{Method, RequestBuilder};
use serde::de::DeserializeOwned;

use crate::client::Auth0Error;
use crate::Auth0;

#[async_trait]
pub trait Auth0Request: Auth0RequestBuilder + Send + Sync {
  async fn send(&self) -> Result<Self::Response, Auth0Error>;
}

#[async_trait]
impl<A: AsRef<Auth0> + Auth0RequestBuilder + Send + Sync> Auth0Request for A {
  async fn send(&self) -> Result<A::Response, Auth0Error> {
    let mut client = self.as_ref().0.lock().await;
    client.query(self).await
  }
}

/// Builds request without absolute URI.
pub trait Auth0RequestBuilder {
  /// The response type.
  type Response: DeserializeOwned;

  /// Build relative request.
  ///
  /// # Arguments
  /// * `factory` - The absolute request builder factory.
  fn build<F>(&self, factory: F) -> RequestBuilder
  where
    F: FnOnce(Method, &str) -> RequestBuilder;
}
