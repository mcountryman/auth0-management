use reqwest::{Method, RequestBuilder};

use crate::request::Auth0Request;

/// Provides data used to delete user using user_id field.
pub struct UpdateUser(String);

impl UpdateUser {
  /// Create delete user request.
  /// # Arguments
  /// * `id` - The id of the user to delete.
  pub fn new(id: &str) -> Self {
    Self(id.to_owned())
  }
}

impl Auth0Request for UpdateUser {
  type Response = ();

  fn build<F>(&self, factory: F) -> RequestBuilder
  where
    F: FnOnce(Method, &str) -> RequestBuilder,
  {
    factory(Method::DELETE, &format!("api/v2/users/{}", self.0))
  }
}
