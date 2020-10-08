use reqwest::{Method, RequestBuilder};
use serde::Serialize;

use crate::api::users::{EmptyAppMetadata, EmptyUserMetadata, User};
use crate::request::Auth0Request;
use serde::de::DeserializeOwned;

/// Provides data to update user.
#[derive(Serialize)]
pub struct UpdateUser<AppMetadata = EmptyAppMetadata, UserMetadata = EmptyUserMetadata> {
  #[serde(skip_serializing)]
  user_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  blocked: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  email: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  email_verified: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  phone_number: Option<String>,
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
  password: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  connection: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  client_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  verify_email: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  verify_phone_number: Option<bool>,

  #[serde(skip_serializing_if = "Option::is_none")]
  app_metadata: Option<AppMetadata>,
  #[serde(skip_serializing_if = "Option::is_none")]
  user_metadata: Option<UserMetadata>,
}

impl<AppMetadata, UserMetadata> UpdateUser<AppMetadata, UserMetadata> {
  /// Create update user request.
  pub fn new(id: &str) -> Self {
    Self {
      user_id: id.to_owned(),
      blocked: None,
      email: None,
      email_verified: None,
      phone_number: None,
      phone_verified: None,
      given_name: None,
      family_name: None,
      name: None,
      nickname: None,
      picture: None,
      verify_email: None,
      verify_phone_number: None,
      password: None,
      connection: None,
      client_id: None,
      app_metadata: None,
      user_metadata: None,
    }
  }

  /// ID of the user which can be used when interacting with other APIs.
  pub fn user_id(&mut self, id: &str) -> &mut Self {
    self.user_id = id.to_owned();
    self
  }

  /// Whether this user was blocked by an administrator (true) or not (false).
  pub fn blocked(&mut self, blocked: bool) -> &mut Self {
    self.blocked = Some(blocked);
    self
  }

  /// Email address of this user.
  pub fn email(&mut self, email: &str) -> &mut Self {
    self.email = Some(email.to_owned());
    self
  }

  /// Whether this email address is verified (true) or unverified (false). If set to false the user
  /// will not receive a verification email unless `verify_email` is set to true.
  pub fn email_verified(&mut self, email_verified: bool) -> &mut Self {
    self.email_verified = Some(email_verified);
    self
  }

  /// The user's phone number (following the E.164 recommendation), only valid for users from SMS
  /// connections.
  pub fn phone_number(&mut self, phone_number: &str) -> &mut Self {
    self.phone_number = Some(phone_number.to_owned());
    self
  }

  /// Whether this phone number has been verified (true) or not (false).
  pub fn phone_verified(&mut self, phone_verified: bool) -> &mut Self {
    self.phone_verified = Some(phone_verified);
    self
  }

  /// Given name/first name/forename of this user.
  pub fn given_name(&mut self, given_name: &str) -> &mut Self {
    self.given_name = Some(given_name.to_owned());
    self
  }

  /// Family name/last name/surname of this user.
  pub fn family_name(&mut self, family_name: &str) -> &mut Self {
    self.family_name = Some(family_name.to_owned());
    self
  }

  /// Name of this user.
  pub fn name(&mut self, name: &str) -> &mut Self {
    self.name = Some(name.to_owned());
    self
  }

  /// Preferred nickname or alias of this user.
  pub fn nickname(&mut self, nickname: &str) -> &mut Self {
    self.nickname = Some(nickname.to_owned());
    self
  }

  /// URL to picture, photo, or avatar of this user.
  pub fn picture(&mut self, picture: &str) -> &mut Self {
    self.picture = Some(picture.to_owned());
    self
  }

  /// Whether this user will receive a verification email after creation (true) or no email (false).
  /// Overrides behavior of `email_verified` parameter.
  pub fn verify_email(&mut self, verify_email: bool) -> &mut Self {
    self.verify_email = Some(verify_email);
    self
  }

  /// Whether this user will receive a text after changing the phone number (true) or no text
  /// (false). Only valid when changing phone number.
  pub fn verify_phone_number(&mut self, verify_phone_number: bool) -> &mut Self {
    self.verify_phone_number = Some(verify_phone_number);
    self
  }

  /// New password for this user (mandatory for non-SMS connections).
  pub fn password(&mut self, password: &str) -> &mut Self {
    self.password = Some(password.to_owned());
    self
  }

  /// ID of the connection this user should be created in.
  pub fn connection(&mut self, connection: &str) -> &mut Self {
    self.connection = Some(connection.to_owned());
    self
  }

  /// Auth0 client ID. Only valid when updating email address.
  pub fn client_id(&mut self, client_id: &str) -> &mut Self {
    self.client_id = Some(client_id.to_owned());
    self
  }

  /// User metadata to which this user has read-only access.
  pub fn app_metadata(&mut self, app_metadata: AppMetadata) -> &mut Self {
    self.app_metadata = Some(app_metadata);
    self
  }

  /// User metadata to which this user has read/write access.
  pub fn user_metadata(&mut self, user_metadata: UserMetadata) -> &mut Self {
    self.user_metadata = Some(user_metadata);
    self
  }
}

impl<
    AppMetadata: Serialize + DeserializeOwned,
    UserMetadata: Serialize + DeserializeOwned,
  > Auth0Request for UpdateUser<AppMetadata, UserMetadata>
{
  type Response = User<AppMetadata, UserMetadata>;

  fn build<F>(&self, factory: F) -> RequestBuilder
  where
    F: FnOnce(Method, &str) -> RequestBuilder,
  {
    factory(Method::DELETE, &format!("api/v2/users/{}", self.user_id)).json(self)
  }
}
