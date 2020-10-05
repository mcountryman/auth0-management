use std::error::Error;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::{ClientRequestBuilder, ManagementClient};
use chrono::{DateTime, Utc};
use std::time::{SystemTime, SystemTimeError};

#[derive(Serialize, Deserialize, Debug)]
pub struct Identity {
  provider: String,
  #[serde(rename = "isSocial")]
  is_social: bool,
  connection: String,
}

#[derive(Deserialize, Debug)]
pub struct User<AppMetadata, UserMetadata> {
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
  page_size: Option<u32>,
  include_totals: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct UserCreateOpts<AppMeta, UserMeta> {
  pub user_id: String,
  pub email: String,
  pub email_verified: bool,
  pub username: String,
  pub phone_number: String,
  pub phone_verified: bool,
  pub picture: String,
  pub name: String,
  pub nickname: String,
  pub last_ip: String,
  pub last_login: String,
  pub logins_count: u32,
  pub blocked: bool,
  pub given_name: String,
  pub family_name: String,

  pub app_metadata: AppMeta,
  pub user_metadata: UserMeta,
}

#[derive(Serialize, Deserialize)]
pub struct UserUpdateOpts<AppMeta, UserMeta> {
  pub user_id: String,
  pub blocked: Option<bool>,
  pub email: Option<String>,
  pub email_verified: Option<bool>,
  pub phone_number: Option<String>,
  pub phone_verified: Option<bool>,
  pub given_name: Option<String>,
  pub family_name: Option<String>,
  pub name: Option<String>,
  pub nickname: Option<String>,
  pub picture: Option<String>,
  pub verify_email: Option<bool>,
  pub verify_phone_number: Option<bool>,

  pub app_metadata: Option<AppMeta>,
  pub user_metadata: Option<UserMeta>,
}

#[async_trait]
pub trait UsersManager {
  async fn get_user<AppMeta: DeserializeOwned, UserMeta: DeserializeOwned>(
    &mut self,
    id: &str,
  ) -> Result<Option<User<AppMeta, UserMeta>>, Box<dyn Error + Send + Sync>>;

  async fn find_users<AppMeta: DeserializeOwned, UserMeta: DeserializeOwned>(
    &mut self,
    opts: &UsersFindOpts,
  ) -> Result<Vec<User<AppMeta, UserMeta>>, Box<dyn Error + Send + Sync>>;

  async fn update_user<
    AppMeta: Serialize + DeserializeOwned + Send + Sync,
    UserMeta: Serialize + DeserializeOwned + Send + Sync,
    U: Into<UserUpdateOpts<AppMeta, UserMeta>> + Send + Sync,
  >(
    &mut self,
    user: U,
  ) -> Result<User<AppMeta, UserMeta>, Box<dyn Error + Send + Sync>>;

  async fn delete_user<AppMeta: DeserializeOwned, UserMeta: DeserializeOwned>(
    &mut self,
    id: &str,
  ) -> Result<(), Box<dyn Error + Send + Sync>>;
}

#[async_trait]
impl UsersManager for ManagementClient {
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
    opts: &UsersFindOpts,
  ) -> Result<Vec<User<AppMeta, UserMeta>>, Box<dyn Error + Send + Sync>> {
    self.get("api/v2/users")?.query(opts).send_json(self).await
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

  async fn delete_user<AppMeta: DeserializeOwned, UserMeta: DeserializeOwned>(
    &mut self,
    id: &str,
  ) -> Result<(), Box<dyn Error + Send + Sync>> {
    self
      .delete(&format!("api/v2/users/{}", id))?
      .send_pass(self)
      .await
  }
}

impl UsersFindOpts {
  pub fn new() -> Self {
    Self {
      page: None,
      page_size: None,
      include_totals: None,
    }
  }

  pub fn page(mut self, page: u32) -> Self {
    self.page = Some(page);
    self
  }

  pub fn page_size(mut self, page_size: u32) -> Self {
    self.page_size = Some(page_size);
    self
  }

  pub fn include_totals(mut self, include_totals: bool) -> Self {
    self.include_totals = Some(include_totals);
    self
  }
}
// impl<AppMeta: Serialize, UserMeta: Serialize> Into<UserUpdateOpts<AppMeta, UserMeta>>
//   for User<AppMeta, UserMeta>
// {
//   fn into(self) -> UserUpdateOpts<AppMeta, UserMeta> {
//     UserUpdateOpts {
//       user_id: self.user_id,
//       blocked: None,
//       email: Some(self.email),
//       email_verified: Some(self.email_verified),
//       phone_number: self.phone_number,
//       phone_verified: Some(self.phone_verified),
//       given_name: Some(self.given_name),
//       family_name: Some(self.family_name),
//       name: Some(self.name),
//       nickname: Some(self.nickname),
//       picture: Some(self.picture),
//       verify_email: None,
//       verify_phone_number: None,
//
//       app_metadata: Some(self.app_metadata),
//       user_metadata: Some(self.user_metadata),
//     }
//   }
// }
