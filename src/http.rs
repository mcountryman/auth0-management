//! Client.

use crate::error::Auth0Result;
use derive_builder::Builder;
use serde::{de::DeserializeOwned, Serialize};

/// A HTTP client that sends requests and reads responses.
#[async_trait::async_trait]
pub trait HttpClient {
  /// Sends an HTTP requests and reads the deserialized JSON response content.
  async fn send<Body, Response>(&self, req: HttpRequest<Body>) -> Auth0Result<Response>
  where
    Body: Send + Sync + Serialize,
    Response: Send + Sync + DeserializeOwned;
}

/// A reqwest HTTP client.
pub struct ReqwestHttpClient {
  url: reqwest::Url,
  client: reqwest::Client,
}

#[async_trait::async_trait]
impl HttpClient for ReqwestHttpClient {
  async fn send<Body, Response>(&self, req: HttpRequest<Body>) -> Auth0Result<Response>
  where
    Body: Send + Sync + Serialize,
    Response: Send + Sync + DeserializeOwned,
  {
    let url = self.url.join(&req.path)?;
    let res = match req.method {
      HttpMethod::Get => self.client.get(url),
      HttpMethod::Put => self.client.put(url),
      HttpMethod::Post => self.client.post(url),
      HttpMethod::Delete => self.client.delete(url),
    }
    .json(&req.body)
    .query(&req.query)
    .send()
    .await?
    .json::<Response>()
    .await?;

    Ok(res)
  }
}

/// The HTTP request method.
#[derive(Clone, Copy)]
pub enum HttpMethod {
  /// The GET method.
  Get,
  /// The POST method.
  Post,
  /// The PUT method.
  Put,
  /// The DELETE method.
  Delete,
}

/// An HTTP request.
#[derive(Builder)]
pub struct HttpRequest<Body> {
  /// The request path.
  #[builder(setter(into))]
  pub path: String,
  /// The request body.
  #[builder(setter(into, strip_option), default)]
  pub body: Option<Body>,
  /// The request query parameters.
  #[builder(setter(custom))]
  pub query: Vec<(String, String)>,
  /// The request method.
  #[builder(setter(into))]
  pub method: HttpMethod,
}

impl HttpRequest<()> {
  /// Create a new [HttpRequestBuilder].
  pub fn builder<Body: Clone>() -> HttpRequestBuilder<Body> {
    HttpRequestBuilder::default()
  }
}

impl<Body> HttpRequestBuilder<Body> {
  /// Appends a query string item to the request builder.
  pub fn query<K: ToString, V: ToString>(&mut self, key: K, value: V) -> &mut Self {
    self
      .query
      .get_or_insert(Vec::default())
      .push((key.to_string(), value.to_string()));
    self
  }
}
#[cfg(test)]
mod tests {
  #[test]
  fn build_request() {
    use super::*;

    let req = HttpRequestBuilder::<()>::default()
      .path("/foo")
      .method(HttpMethod::Put)
      .query("foo", "bar")
      .query("baz", "qux")
      .build()
      .unwrap();

    assert_eq!(req.path, "/foo");
    assert_eq!(
      req.query,
      vec![
        ("foo".to_string(), "bar".to_string()),
        ("baz".to_string(), "qux".to_string())
      ]
    );
  }
}
