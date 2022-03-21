#![doc = include_str!("../README.md")]
#![deny(unsafe_code)]
#![deny(missing_docs)]

use error::Auth0Result;
use http::HttpRequest;
use std::sync::Arc;

pub mod entities;
pub mod error;
pub mod http;

/// An Auth0 management client.
pub struct Auth0 {}

/// An Auth0 request.
#[async_trait::async_trait]
pub trait Auth0Request<Client>: Sized {
  /// The request body type.
  type Body;
  /// The response type.
  type Response;

  /// Gets the request [Client].
  fn client(&self) -> Auth0Result<Arc<Client>>;

  /// Attempts to get a [HttpRequest].
  fn to_request(&self) -> Auth0Result<HttpRequest<Self::Body>>;
}
