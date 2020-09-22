use reqwest::{Client, ClientBuilder, Method, RequestBuilder, Result};

pub struct Api {
  base_uri: String,
  config_builder: Option<Box<dyn FnOnce(ClientBuilder) -> Result<Client>>>,
}

impl Api {
  pub fn new() -> Self {
    Self {
      base_uri: "".to_owned(),
      config_builder: None,
    }
  }

  pub fn request(&self, method: Method, path: &str) -> Result<RequestBuilder> {
    Ok(
      Client::builder()
        .build()?
        .request(method, &format!("{}/{}", self.base_uri, path)),
    )
  }
}
