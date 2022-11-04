//! Trigger password reset.
use reqwest::{Method, RequestBuilder};
use serde::Serialize;

use crate::{Auth0Client, Auth0RequestBuilder};

/// Trigger password reset.
#[derive(Serialize)]
pub struct UserResetPassword {
  email: String,
  connection: String,
  client_id: String,
}

impl UserResetPassword {
  /// Create reset password request.
  pub fn new(email: &str, connection: &str, client_id: &str) -> Self {
    Self {
      email: email.to_owned(),
      connection: connection.to_owned(),
      client_id: client_id.to_owned(),
    }
  }
}

impl Auth0RequestBuilder for UserResetPassword {
  fn build(&self, client: &Auth0Client) -> RequestBuilder {
    client
      .begin(Method::POST, "dbconnections/change_password")
      .json(self)
  }
}
