use reqwest::{Client, Error, Method, RequestBuilder};

use super::Auth0Driver;

#[derive(Clone)]
pub struct DefaultAuth0Driver {
  client: Client,
}

#[async_trait]
impl Auth0Driver for DefaultAuth0Driver {
  type Error = Error;
  type Method = Method;
  type Builder = RequestBuilder;

  fn build<U: AsRef<str>>(&self, method: Self::Method, url: U) -> Self::Builder {
    self.client.request(method, url.as_ref())
  }
}
