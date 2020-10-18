//! Delete a user.
use reqwest::{Method, RequestBuilder};

use crate::{Auth0Client, Auth0RequestBuilder};

/// Delete a user.
///
/// # Scopes
/// * `delete:users`
pub struct UserDelete(String);

impl UserDelete {
  /// Create delete user request.
  pub fn new<S: AsRef<str>>(id: S) -> Self {
    Self(id.as_ref().to_string())
  }
}

impl Auth0RequestBuilder for UserDelete {
  fn build(&self, client: &Auth0Client) -> RequestBuilder {
    client.begin(Method::DELETE, &format!("api/v2/users/{}", self.0))
  }
}
