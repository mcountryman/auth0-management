use std::error::Error;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::{ClientRequestBuilder, ManagementClient};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct Identity {
  provider: String,
  #[serde(rename = "isSocial")]
  is_social: bool,
  connection: String,
}

#[derive(Deserialize, Debug)]
pub struct User<AppMetadata = (), UserMetadata = ()> {
  pub user_id: String,
  pub email: String,
  pub email_verified: bool,
  pub username: Option<String>,
  pub phone_number: Option<String>,
  #[serde(default)]
  pub phone_verified: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub identities: Vec<Identity>,
  pub picture: String,
  pub name: String,
  pub nickname: String,
  pub last_ip: Option<String>,
  pub last_login: Option<DateTime<Utc>>,
  #[serde(default)]
  pub logins_count: u32,
  #[serde(default)]
  pub blocked: bool,
  pub given_name: Option<String>,
  pub family_name: Option<String>,
  pub app_metadata: Option<AppMetadata>,
  pub user_metadata: Option<UserMetadata>,
}

#[derive(Serialize)]
pub struct UsersFindOpts {
  page: Option<u32>,
  #[serde(rename = "per_page")]
  page_size: Option<u8>,
  include_totals: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct UserCreateOpts<AppMeta = (), UserMeta = ()> {
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
  app_metadata: Option<AppMeta>,
  #[serde(skip_serializing_if = "Option::is_none")]
  user_metadata: Option<UserMeta>,
}

#[async_trait]
pub trait UsersManager {
  async fn get_user<AppMeta: DeserializeOwned, UserMeta: DeserializeOwned>(
    &mut self,
    id: &str,
  ) -> Result<Option<User<AppMeta, UserMeta>>, Box<dyn Error + Send + Sync>>;

  async fn find_users<AppMeta: DeserializeOwned, UserMeta: DeserializeOwned>(
    &mut self,
    opts: UsersFindOpts,
  ) -> Result<Vec<User<AppMeta, UserMeta>>, Box<dyn Error + Send + Sync>>;

  async fn create_user<
    AppMeta: Serialize + DeserializeOwned + Send + Sync,
    UserMeta: Serialize + DeserializeOwned + Send + Sync,
  >(
    &mut self,
    user: UserCreateOpts<AppMeta, UserMeta>,
  ) -> Result<User<AppMeta, UserMeta>, Box<dyn Error + Send + Sync>>;

  async fn update_user<
    AppMeta: Serialize + DeserializeOwned + Send + Sync,
    UserMeta: Serialize + DeserializeOwned + Send + Sync,
    U: Into<UserUpdateOpts<AppMeta, UserMeta>> + Send + Sync,
  >(
    &mut self,
    user: U,
  ) -> Result<User<AppMeta, UserMeta>, Box<dyn Error + Send + Sync>>;

  async fn delete_user(&mut self, id: &str) -> Result<(), Box<dyn Error + Send + Sync>>;
}

#[async_trait]
impl UsersManager for ManagementClient {
  /// Retrieve user details.
  /// # Arguments
  /// * `id` - The id of the user.
  async fn get_user<AppMeta: DeserializeOwned, UserMeta: DeserializeOwned>(
    &mut self,
    id: &str,
  ) -> Result<Option<User<AppMeta, UserMeta>>, Box<dyn Error + Send + Sync>> {
    self
      .get(&format!("api/v2/users/{}", id))?
      .send_json(self)
      .await
  }

  async fn find_users<AppMeta: DeserializeOwned, UserMeta: DeserializeOwned>(
    &mut self,
    opts: UsersFindOpts,
  ) -> Result<Vec<User<AppMeta, UserMeta>>, Box<dyn Error + Send + Sync>> {
    self.get("api/v2/users")?.query(&opts).send_json(self).await
  }

  async fn create_user<
    AppMeta: Serialize + DeserializeOwned + Send + Sync,
    UserMeta: Serialize + DeserializeOwned + Send + Sync,
  >(
    &mut self,
    user: UserCreateOpts<AppMeta, UserMeta>,
  ) -> Result<User<AppMeta, UserMeta>, Box<dyn Error + Send + Sync>>
  where
    AppMeta: Serialize + DeserializeOwned + Send + Sync,
  {
    self.post("api/v2/users")?.json(&user).send_json(self).await
  }

  async fn update_user<
    AppMeta: Serialize + DeserializeOwned + Send + Sync,
    UserMeta: Serialize + DeserializeOwned + Send + Sync,
    U: Into<UserUpdateOpts<AppMeta, UserMeta>> + Send + Sync,
  >(
    &mut self,
    user: U,
  ) -> Result<User<AppMeta, UserMeta>, Box<dyn Error + Send + Sync>> {
    let user = user.into();

    self
      .patch(&format!("api/v2/users/{}", user.user_id))?
      .json(&user)
      .send_json(self)
      .await
  }

  async fn delete_user(&mut self, id: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    self
      .delete(&format!("api/v2/users/{}", id))?
      .send_pass(self)
      .await
  }
}

impl<AppMeta, UserMeta> UserCreateOpts<AppMeta, UserMeta> {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn email(mut self, email: &str) -> Self {
    self.email = Some(email.to_owned());
    self
  }
  pub fn phone_number(mut self, phone_number: &str) -> Self {
    self.phone_number = Some(phone_number.to_owned());
    self
  }
  pub fn blocked(mut self, blocked: bool) -> Self {
    self.blocked = Some(blocked);
    self
  }
  pub fn email_verified(mut self, email_verified: bool) -> Self {
    self.email_verified = Some(email_verified);
    self
  }
  pub fn phone_verified(mut self, phone_verified: bool) -> Self {
    self.phone_verified = Some(phone_verified);
    self
  }
  pub fn given_name(mut self, given_name: &str) -> Self {
    self.given_name = Some(given_name.to_owned());
    self
  }
  pub fn family_name(mut self, family_name: &str) -> Self {
    self.family_name = Some(family_name.to_owned());
    self
  }
  pub fn name(mut self, name: &str) -> Self {
    self.name = Some(name.to_owned());
    self
  }
  pub fn nickname(mut self, nickname: &str) -> Self {
    self.nickname = Some(nickname.to_owned());
    self
  }
  pub fn picture(mut self, picture: &str) -> Self {
    self.picture = Some(picture.to_owned());
    self
  }
  pub fn user_id(mut self, user_id: &str) -> Self {
    self.user_id = Some(user_id.to_owned());
    self
  }
  pub fn connection(mut self, connection: &str) -> Self {
    self.connection = Some(connection.to_owned());
    self
  }
  pub fn password(mut self, password: &str) -> Self {
    self.password = Some(password.to_owned());
    self
  }
  pub fn verify_email(mut self, verify_email: bool) -> Self {
    self.verify_email = Some(verify_email);
    self
  }
  pub fn username(mut self, username: &str) -> Self {
    self.username = Some(username.to_owned());
    self
  }

  pub fn app_metadata(mut self, app_metadata: AppMeta) -> Self {
    self.app_metadata = Some(app_metadata);
    self
  }

  pub fn user_metadata(mut self, user_metadata: UserMeta) -> Self {
    self.user_metadata = Some(user_metadata);
    self
  }
}

impl<AppMeta, UserMeta> Default for UserCreateOpts<AppMeta, UserMeta> {
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
      app_metadata: None,
      user_metadata: None,
    }
  }
}

impl UsersFindOpts {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn page(mut self, page: u32) -> Self {
    self.page = Some(page);
    self
  }

  pub fn page_size(mut self, page_size: u8) -> Self {
    self.page_size = Some(page_size);
    self
  }

  pub fn include_totals(mut self, include_totals: bool) -> Self {
    self.include_totals = Some(include_totals);
    self
  }
}

impl Default for UsersFindOpts {
  fn default() -> Self {
    Self {
      page: None,
      page_size: None,
      include_totals: None,
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct UserUpdateOpts<AppMeta, UserMeta> {
  #[serde(skip_serializing)]
  pub user_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub blocked: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub phone_number: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub given_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub family_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nickname: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub picture: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub verify_email: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub verify_phone_number: Option<bool>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub app_metadata: Option<AppMeta>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user_metadata: Option<UserMeta>,
}

impl<AppMetadata, UserMetadata> Into<UserUpdateOpts<AppMetadata, UserMetadata>>
  for User<AppMetadata, UserMetadata>
{
  fn into(self) -> UserUpdateOpts<AppMetadata, UserMetadata> {
    UserUpdateOpts {
      user_id: self.user_id,
      blocked: Some(self.blocked),
      email: Some(self.email),
      phone_number: self.phone_number,
      given_name: self.given_name,
      family_name: self.family_name,
      name: Some(self.name),
      nickname: Some(self.nickname),
      picture: Some(self.picture),
      verify_email: None,
      verify_phone_number: None,
      app_metadata: None,
      user_metadata: None,
    }
  }
}
