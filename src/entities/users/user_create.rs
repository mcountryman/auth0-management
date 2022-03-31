//! User create request.

use derive_builder::Builder;
use serde::Serialize;

/// Create a new user for a given [database](https://auth0.com/docs/connections/database) or
/// [passwordless](https://auth0.com/docs/connections/passwordless) connection.
#[derive(Serialize, Clone, Builder)]
pub struct UserCreate<A, U> {
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<String>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub phone_number: Option<String>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub blocked: Option<bool>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub email_verified: Option<bool>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub phone_verified: Option<bool>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub given_name: Option<String>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub family_name: Option<String>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nickname: Option<String>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub picture: Option<String>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user_id: Option<String>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub connection: Option<String>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub password: Option<String>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub verify_email: Option<bool>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub username: Option<String>,

  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub app_metadata: Option<A>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user_metadata: Option<U>,
}

impl<A: Clone, U: Clone> TryFrom<UserCreateBuilder<A, U>> for UserCreate<A, U> {
  type Error = UserCreateBuilderError;

  fn try_from(builder: UserCreateBuilder<A, U>) -> Result<Self, Self::Error> {
    builder.build()
  }
}
