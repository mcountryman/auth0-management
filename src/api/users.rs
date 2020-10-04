use crate::client::{Client, ClientRequestBuilder};
use crate::ManagementClient;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct Identity {
  provider: String,
  #[serde(rename = "isSocial")]
  is_social: bool,
  connection: String,
}

#[derive(Deserialize)]
pub struct User<AppMetadata, UserMetadata> {
  user_id: String,
  email: String,
  email_verified: bool,
  username: String,
  phone_number: String,
  phone_verified: bool,
  created_at: String,
  updated_at: String,
  identities: Vec<Identity>,
  picture: String,
  name: String,
  nickname: String,
  last_ip: String,
  last_login: String,
  logins_count: u32,
  blocked: bool,
  given_name: String,
  family_name: String,

  app_metadata: AppMetadata,
  user_metadata: UserMetadata,
}

#[derive(Serialize)]
pub struct UsersFindOpts {
  page: Option<u32>,
  page_size: Option<u32>,
  include_totals: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct UserCreateOpts<AppMeta, UserMeta> {
  user_id: String,
  email: String,
  email_verified: bool,
  username: String,
  phone_number: String,
  phone_verified: bool,
  picture: String,
  name: String,
  nickname: String,
  last_ip: String,
  last_login: String,
  logins_count: u32,
  blocked: bool,
  given_name: String,
  family_name: String,

  app_metadata: AppMeta,
  user_metadata: UserMeta,
}

#[derive(Serialize, Deserialize)]
pub struct UserUpdateOpts<AppMeta, UserMeta> {
  user_id: String,
  blocked: Option<bool>,
  email: Option<String>,
  email_verified: Option<bool>,
  phone_number: Option<String>,
  phone_verified: Option<bool>,
  given_name: Option<String>,
  family_name: Option<String>,
  name: Option<String>,
  nickname: Option<String>,
  picture: Option<String>,
  verify_email: Option<bool>,
  verify_phone_number: Option<bool>,

  app_metadata: Option<AppMeta>,
  user_metadata: Option<UserMeta>,
}

#[async_trait]
pub trait UsersManager {
  async fn get_user<AppMeta: DeserializeOwned, UserMeta: DeserializeOwned>(
    &mut self,
    id: &str,
  ) -> Result<User<AppMeta, UserMeta>, Box<dyn Error + Send + Sync>>;

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
  ) -> Result<User<AppMeta, UserMeta>, Box<dyn Error + Send + Sync>> {
    self
      .client
      .get(&format!("/api/v2/user/{}", id))?
      .send_json(&mut self.client)
      .await
  }

  async fn find_users<AppMeta: DeserializeOwned, UserMeta: DeserializeOwned>(
    &mut self,
    opts: &UsersFindOpts,
  ) -> Result<Vec<User<AppMeta, UserMeta>>, Box<dyn Error + Send + Sync>> {
    self
      .client
      .get("/api/v2/users")?
      .query(opts)
      .send_json(&mut self.client)
      .await
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
      .client
      .patch(&format!("/api/v2/users/{}", user.user_id))?
      .json(&user)
      .send_json(&mut self.client)
      .await
  }

  async fn delete_user<AppMeta: DeserializeOwned, UserMeta: DeserializeOwned>(
    &mut self,
    id: &str,
  ) -> Result<(), Box<dyn Error + Send + Sync>> {
    self
      .client
      .delete(&format!("/api/v2/users/{}", id))?
      .send_pass(&mut self.client)
      .await
  }
}

impl<AppMeta: Serialize, UserMeta: Serialize> Into<UserUpdateOpts<AppMeta, UserMeta>>
  for User<AppMeta, UserMeta>
{
  fn into(self) -> UserUpdateOpts<AppMeta, UserMeta> {
    UserUpdateOpts {
      user_id: self.user_id,
      blocked: None,
      email: Some(self.email),
      email_verified: Some(self.email_verified),
      phone_number: Some(self.phone_number),
      phone_verified: Some(self.phone_verified),
      given_name: Some(self.given_name),
      family_name: Some(self.family_name),
      name: Some(self.name),
      nickname: Some(self.nickname),
      picture: Some(self.picture),
      verify_email: None,
      verify_phone_number: None,

      app_metadata: Some(self.app_metadata),
      user_metadata: Some(self.user_metadata),
    }
  }
}
