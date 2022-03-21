//! Retrieve user details. A list of fields to include or exclude may also be specified.
use reqwest::{Method, RequestBuilder};

use crate::{Auth0Client, Auth0RequestBuilder};

/// Retrieve user details. A list of fields to include or exclude may also be specified.
pub struct UserGet {
  id: String,
}

impl UserGet {
  /// Create get user request.
  pub fn new<S: AsRef<str>>(id: S) -> Self {
    Self {
      id: id.as_ref().to_string(),
    }
  }
}

impl Auth0RequestBuilder for UserGet {
  fn build(&self, client: &Auth0Client) -> RequestBuilder {
    client.begin(Method::GET, &format!("api/v2/users/{}", self.id))
  }
}

/// Provides data used to request user from email field.
pub struct GetUserByEmail {
  email: String,
}

impl GetUserByEmail {
  /// Create get user request.
  /// # Arguments
  /// * `email` - The email address of the user to retrieve.
  pub fn new(email: &str) -> Self {
    Self {
      email: email.to_owned(),
    }
  }
}

impl Auth0RequestBuilder for GetUserByEmail {
  fn build(&self, client: &Auth0Client) -> RequestBuilder {
    client
      .begin(Method::GET, "api/v2/users/users-by-email")
      .query(&[("email", self.email.to_owned())])
  }
}
