#![doc = include_str!("../README.md")]
#![deny(unsafe_code)]
#![deny(missing_docs)]

use error::Auth0Result;
use reqwest_middleware::{ClientWithMiddleware, RequestBuilder};
use url::Url;

pub mod entities;
pub mod error;
pub mod http;

/// An Auth0 management client.
pub struct Auth0 {
  url: Url,
  client: ClientWithMiddleware,
}

impl Auth0 {
  pub(crate) fn post(&self, path: impl AsRef<str>) -> Auth0Result<RequestBuilder> {
    let url = self.url.join(path.as_ref())?;
    Ok(self.client.post(url))
  }
}
