use reqwest::{Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::export::PhantomData;
use serde::Serialize;

use crate::api::users::{EmptyAppMetadata, EmptyUserMetadata, User};
use crate::request::Auth0Request;

#[derive(Serialize)]
pub struct FindUsers<AppMetadata = EmptyAppMetadata, UserMetadata = EmptyUserMetadata> {
  page: Option<u32>,
  #[serde(rename = "per_page")]
  page_size: Option<u8>,
  include_totals: Option<bool>,

  app_metadata: PhantomData<AppMetadata>,
  user_metadata: PhantomData<UserMetadata>,
}

impl<AppMetadata, UserMetadata> FindUsers<AppMetadata, UserMetadata> {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn page(&mut self, page: u32) -> &mut Self {
    self.page = Some(page);
    self
  }

  pub fn page_size(&mut self, page_size: u8) -> &mut Self {
    self.page_size = Some(page_size);
    self
  }

  pub fn include_totals(&mut self, include_totals: bool) -> &mut Self {
    self.include_totals = Some(include_totals);
    self
  }
}

impl<AppMetadata, UserMetadata> Default for FindUsers<AppMetadata, UserMetadata> {
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
  for FindUsers<AppMetadata, UserMetadata>
{
  type Response = Vec<User<AppMetadata, UserMetadata>>;

  fn build<F>(&self, factory: F) -> RequestBuilder
  where
    F: FnOnce(Method, &str) -> RequestBuilder,
  {
    factory(Method::GET, "api/v2/users").query(self)
  }
}
