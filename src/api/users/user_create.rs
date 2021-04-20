//! Create a new user.
use crate::impl_request;

use super::User;

/// Create a new user for a given [database](https://auth0.com/docs/connections/database) or
/// [passwordless](https://auth0.com/docs/connections/passwordless) connection.
#[derive(Serialize, Builder, Clone, Debug)]
pub struct UserCreate<A, U, D> {
  #[serde(skip_serializing)]
  client: Option<D>,

  #[serde(skip_serializing_if = "Option::is_none")]
  email: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  phone_number: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  blocked: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  email_verified: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  phone_verified: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  given_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  family_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  nickname: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  picture: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  user_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  connection: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  password: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  verify_email: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  username: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  app_metadata: Option<A>,
  #[serde(skip_serializing_if = "Option::is_none")]
  user_metadata: Option<U>,
}

impl_request!(UserCreate, UserCreateBuilder, User, |req, driver| {
  driver.build(Method::POST, "api/v2/users").json(&req)
});
