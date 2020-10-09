//! Delete a user.
use reqwest::{Method, RequestBuilder};

use crate::Auth0Request;

/// Delete a user.
///
/// # Scopes
/// * `delete:users`
pub struct UserDelete(String);

impl UserDelete {
  /// Create delete user request.
  ///
  /// # Arguments
  /// * `id` - The id of the user to delete.
  pub fn new(id: &str) -> Self {
    Self(id.to_owned())
  }
}

impl Auth0Request for UserDelete {
  type Response = ();

  fn build<F>(&self, factory: F) -> RequestBuilder
  where
    F: FnOnce(Method, &str) -> RequestBuilder,
  {
    factory(Method::DELETE, &format!("api/v2/users/{}", self.0))
  }
}
