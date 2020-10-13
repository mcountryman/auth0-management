//! Delete a user.
use reqwest::{Method, RequestBuilder};

use crate::Auth0RequestBuilder;

/// Delete a user.
///
/// # Scopes
/// * `delete:users`
pub struct UserDelete(String);

impl UserDelete {
  /// Create delete user request.
  pub fn new<S: AsRef<String>>(id: S) -> Self {
    Self(id.as_ref().to_string())
  }
}

impl Auth0RequestBuilder for UserDelete {
  type Response = ();

  fn build<F>(&self, factory: F) -> RequestBuilder
  where
    F: FnOnce(Method, &str) -> RequestBuilder,
  {
    factory(Method::DELETE, &format!("api/v2/users/{}", self.0))
  }
}
