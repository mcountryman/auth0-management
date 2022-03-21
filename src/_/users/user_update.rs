//! Update a user.
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::users::User;
use crate::{Auth0Client, Auth0Result};

/// Update a user.
#[derive(Serialize)]
pub struct UserUpdate<'a, A, U> {
  #[serde(skip_serializing)]
  client: &'a Auth0Client,

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
  app_metadata: Option<A>,
  #[serde(skip_serializing_if = "Option::is_none")]
  user_metadata: Option<U>,
}

impl<'a> UserUpdate<'a, (), ()> {
  /// Create update user request.
  pub fn new<S: AsRef<str>>(client: &'a Auth0Client, id: S) -> Self {
    Self {
      client,

      user_id: id.as_ref().to_string(),
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
}

impl<'a, A: Clone, U: Clone> UserUpdate<'a, A, U> {
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
  pub fn app_metadata<AppMetadata>(
    &self,
    app_metadata: AppMetadata,
  ) -> UserUpdate<'a, AppMetadata, U> {
    UserUpdate {
      client: self.client,
      user_id: self.user_id.clone(),
      blocked: self.blocked,
      email: self.email.clone(),
      email_verified: self.email_verified,
      phone_number: self.phone_number.clone(),
      phone_verified: self.phone_verified,
      given_name: self.given_name.clone(),
      family_name: self.family_name.clone(),
      name: self.name.clone(),
      nickname: self.nickname.clone(),
      picture: self.picture.clone(),
      password: self.password.clone(),
      connection: self.connection.clone(),
      client_id: self.client_id.clone(),
      verify_email: self.verify_email,
      verify_phone_number: self.verify_phone_number,
      app_metadata: Some(app_metadata),
      user_metadata: self.user_metadata.clone(),
    }
  }

  /// User metadata to which this user has read/write access.
  pub fn user_metadata<UserMetadata>(
    &mut self,
    user_metadata: UserMetadata,
  ) -> UserUpdate<'a, A, UserMetadata> {
    UserUpdate {
      client: self.client,
      user_id: self.user_id.clone(),
      blocked: self.blocked,
      email: self.email.clone(),
      email_verified: self.email_verified,
      phone_number: self.phone_number.clone(),
      phone_verified: self.phone_verified,
      given_name: self.given_name.clone(),
      family_name: self.family_name.clone(),
      name: self.name.clone(),
      nickname: self.nickname.clone(),
      picture: self.picture.clone(),
      password: self.password.clone(),
      connection: self.connection.clone(),
      client_id: self.client_id.clone(),
      verify_email: self.verify_email,
      verify_phone_number: self.verify_phone_number,
      app_metadata: self.app_metadata.clone(),
      user_metadata: Some(user_metadata),
    }
  }
}

impl<'a, AIn, UIn> UserUpdate<'a, AIn, UIn> {
  /// Send
  pub async fn send<AOut, UOut>(&self) -> Auth0Result<User<AOut, UOut>>
  where
    AIn: Serialize,
    UIn: Serialize,
    AOut: DeserializeOwned,
    UOut: DeserializeOwned,
  {
    self
      .client
      .send(
        self
          .client
          .begin(Method::PATCH, &format!("api/v2/users/{}", self.user_id))
          .json(self),
      )
      .await
  }
}
