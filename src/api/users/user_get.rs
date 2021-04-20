//! Retrieve user details. A list of fields to include or exclude may also be specified.
use reqwest::{Method, RequestBuilder};
use crate::impl_request;

/// Retrieve user details. A list of fields to include or exclude may also be specified.
pub struct UserGet(String);

impl UserGet {
  /// Create get user request.
  pub fn new<S: AsRef<str>>(id: S) -> Self {
    Self(id.as_ref().to_string())
  }
}

impl_request!(UserGet, User, |req, driver| {
  driver.build(Method::GET, &format!("api/v2/users/{}", self.0))
});

/// Provides data used to request user from email field.
pub struct GetUserByEmail(String);

impl GetUserByEmail {
  /// Create get user request.
  /// # Arguments
  /// * `email` - The email address of the user to retrieve.
  pub fn new(email: &str) -> Self {
    Self(email.to_owned())
  }
}

impl_request!(GetUserByEmail, User, |req, driver| {
  driver
    .build(Method::GET, "api/v2/users/users-by-email")
    .query(&[("email", self.0.clone())])
});
