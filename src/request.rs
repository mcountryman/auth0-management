use reqwest::{Method, RequestBuilder};
use serde::de::DeserializeOwned;

pub trait Auth0Request {
  type Response: DeserializeOwned;

  fn build<F>(&self, factory: F) -> RequestBuilder
  where
    F: FnOnce(Method, &str) -> RequestBuilder;
}
