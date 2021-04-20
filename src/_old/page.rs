//! Paging helper.
use serde::Serialize;

/// Provides serializable pagination parameters.
#[derive(Clone, Debug, Serialize)]
pub struct Page {
  #[serde(skip_serializing_if = "Option::is_none")]
  page: Option<u32>,
  #[serde(rename = "per_page")]
  #[serde(skip_serializing_if = "Option::is_none")]
  per_page: Option<u32>,
}

impl Page {
  /// Create pagination instance.
  pub fn new() -> Self {
    Default::default()
  }
}

impl Default for Page {
  fn default() -> Self {
    Self {
      page: None,
      per_page: None,
    }
  }
}

/// Build pagination parameters.
pub trait Pageable {
  /// Page index of the results to return.  First page is `0`.
  fn page(&mut self, page: u32) -> &mut Self;
  /// Number of results per page.  Paging is disabled if parameter is `Option::None`
  fn per_page(&mut self, per_page: u32) -> &mut Self;
}

impl<P: AsMut<Page>> Pageable for P {
  fn page(&mut self, page: u32) -> &mut Self {
    self.as_mut().page = Some(page);
    self
  }

  fn per_page(&mut self, per_page: u32) -> &mut Self {
    self.as_mut().per_page = Some(per_page);
    self
  }
}
