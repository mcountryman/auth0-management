use reqwest::{Method, RequestBuilder};
use serde::export::PhantomData;
use serde::Serialize;

use crate::api::users::User;
use crate::request::Auth0Request;
use serde::de::DeserializeOwned;

#[derive(Serialize)]
pub struct FindUsersRequest<AppMetadata = (), UserMetadata = ()> {
  page: Option<u32>,
  #[serde(rename = "per_page")]
  page_size: Option<u8>,
  include_totals: Option<bool>,

  app_metadata: PhantomData<AppMetadata>,
  user_metadata: PhantomData<UserMetadata>,
}

impl<AppMetadata, UserMetadata> FindUsersRequest<AppMetadata, UserMetadata> {
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

impl<AppMetadata, UserMetadata> Default for FindUsersRequest<AppMetadata, UserMetadata> {
  fn default() -> Self {
    Self {
      page: None,
      page_size: None,
      include_totals: None,

      app_metadata: Default::default(),
      user_metadata: Default::default(),
    }
  }
}

impl<AppMetadata: DeserializeOwned, UserMetadata: DeserializeOwned> Auth0Request
  for FindUsersRequest<AppMetadata, UserMetadata>
{
  type Response = Vec<User<AppMetadata, UserMetadata>>;

  fn build<F>(&self, factory: F) -> RequestBuilder
  where
    F: FnOnce(Method, &str) -> RequestBuilder,
  {
    factory(Method::GET, "api/v2/users").query(self)
  }
}
