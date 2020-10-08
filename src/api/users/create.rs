use crate::api::users::{EmptyAppMetadata, EmptyUserMetadata, User};
use crate::request::Auth0Request;
use reqwest::{Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Serialize)]
pub struct CreateUser<AppMetadata = EmptyAppMetadata, UserMetadata = EmptyUserMetadata> {
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
  app_metadata: Option<AppMetadata>,
  #[serde(skip_serializing_if = "Option::is_none")]
  user_metadata: Option<UserMetadata>,
}

impl<AppMetadata, UserMetadata> CreateUser<AppMetadata, UserMetadata> {
  pub fn new() -> Self {
    Default::default()
  }

  /// The user's email.
  pub fn email(&mut self, email: &str) -> &mut Self {
    self.email = Some(email.to_owned());
    self
  }

  /// The user's phone number (following the E.164 recommendation), only valid for users from SMS
  /// connections.
  pub fn phone_number(&mut self, phone_number: &str) -> &mut Self {
    self.phone_number = Some(phone_number.to_owned());
    self
  }

  /// Whether this user was blocked by an administrator (true) or not (false).
  pub fn blocked(&mut self, blocked: bool) -> &mut Self {
    self.blocked = Some(blocked);
    self
  }

  /// Whether this email address is verified (true) or unverified (false). User will receive a
  /// verification email after creation if email_verified is false or not specified.
  pub fn email_verified(&mut self, email_verified: bool) -> &mut Self {
    self.email_verified = Some(email_verified);
    self
  }

  /// Whether this phone number has been verified (true) or not (false).
  pub fn phone_verified(&mut self, phone_verified: bool) -> &mut Self {
    self.phone_verified = Some(phone_verified);
    self
  }

  /// The user's given name(s).
  pub fn given_name(&mut self, given_name: &str) -> &mut Self {
    self.given_name = Some(given_name.to_owned());
    self
  }

  /// The user's family name(s).
  pub fn family_name(&mut self, family_name: &str) -> &mut Self {
    self.family_name = Some(family_name.to_owned());
    self
  }

  /// The user's full name.
  pub fn name(&mut self, name: &str) -> &mut Self {
    self.name = Some(name.to_owned());
    self
  }

  /// The user's nickname.
  pub fn nickname(&mut self, nickname: &str) -> &mut Self {
    self.nickname = Some(nickname.to_owned());
    self
  }

  /// A URI pointing to the user's picture.
  pub fn picture(&mut self, picture: &str) -> &mut Self {
    self.picture = Some(picture.to_owned());
    self
  }

  /// The external user's id provided by the identity provider.
  pub fn user_id(&mut self, user_id: &str) -> &mut Self {
    self.user_id = Some(user_id.to_owned());
    self
  }

  /// Name of the connection this user should be created in.
  pub fn connection(&mut self, connection: &str) -> &mut Self {
    self.connection = Some(connection.to_owned());
    self
  }

  /// Initial password for this user (mandatory for non-SMS connections).
  pub fn password(&mut self, password: &str) -> &mut Self {
    self.password = Some(password.to_owned());
    self
  }

  /// Whether the user will receive a verification email after creation (true) or no email (false).
  /// Overrides behavior of email_verified parameter.
  pub fn verify_email(&mut self, verify_email: bool) -> &mut Self {
    self.verify_email = Some(verify_email);
    self
  }

  /// The user's username. Only valid if the connection requires a username.
  pub fn username(&mut self, username: &str) -> &mut Self {
    self.username = Some(username.to_owned());
    self
  }

  /// Data related to the user that does affect the application's core functionality.
  pub fn app_metadata(&mut self, app_metadata: AppMetadata) -> &mut Self {
    self.app_metadata = Some(app_metadata);
    self
  }

  /// Data related to the user that does not affect the application's core functionality.
  pub fn user_metadata(&mut self, user_metadata: UserMetadata) -> &mut Self {
    self.user_metadata = Some(user_metadata);
    self
  }
}

impl<AppMetadata, UserMetadata> Default for CreateUser<AppMetadata, UserMetadata> {
  fn default() -> Self {
    Self {
      email: None,
      phone_number: None,
      blocked: None,
      email_verified: None,
      phone_verified: None,
      given_name: None,
      family_name: None,
      name: None,
      nickname: None,
      picture: None,
      user_id: None,
      connection: None,
      password: None,
      verify_email: None,
      username: None,

      app_metadata: Default::default(),
      user_metadata: Default::default(),
    }
  }
}

impl<
    AppMetadata: Serialize + DeserializeOwned,
    UserMetadata: Serialize + DeserializeOwned,
  > Auth0Request for CreateUser<AppMetadata, UserMetadata>
{
  type Response = User<AppMetadata, UserMetadata>;

  fn build<F>(&self, factory: F) -> RequestBuilder
  where
    F: FnOnce(Method, &str) -> RequestBuilder,
  {
    factory(Method::POST, "api/v2/users").json(&self)
  }
}
