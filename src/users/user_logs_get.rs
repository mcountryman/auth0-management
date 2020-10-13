//! Retrieve log events for a specific user.
use chrono::{DateTime, Utc};
use reqwest::{Method, RequestBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Auth0Client, Auth0RequestBuilder};
use crate::{Page, Sort};

/// User log event.
#[derive(Debug, Deserialize)]
pub struct UserLog {
  /// Date when the event occurred.
  pub date: DateTime<Utc>,
  /// Type of event.
  #[serde(rename = "type")]
  pub kind: String,
  /// Description of this event.
  pub description: String,
  /// Name of the connection the event relates to.
  pub connection: String,
  /// ID of the connection the event relates to.
  pub connection_id: String,
  /// ID of the client (application).
  pub client_id: String,
  /// Name of the client (application).
  pub client_name: String,
  /// IP address of the log event source.
  pub ip: String,
  /// Hostname the event applies to.
  pub hostname: Option<String>,
  /// ID of the user involved in the event.
  pub user_id: String,
  /// Name of the user involved in the event.
  pub user_name: String,
  /// API audience the event applies to.
  pub audience: Option<String>,
  /// Scope permissions applied to the event.
  pub scope: Option<String>,
  /// Name of the strategy involved in the event.
  pub strategy: String,
  /// Type of strategy involved in the event.
  pub strategy_type: String,
  /// Unique ID of the event.
  pub log_id: String,
  /// Whether the client was a mobile device (true) or desktop/laptop/server (false).
  #[serde(rename = "isMobile")]
  pub is_mobile: bool,
  /// User agent string from the client device that caused the event.
  pub user_agent: String,
  /// Additional useful details about this event (structure is dependent upon event type).
  pub details: Value,
  /// Information about the location that triggered this event based on the ip.
  pub location_info: UserLogLocationInfo,
}

/// User log event location.
#[derive(Debug, Deserialize)]
pub struct UserLogLocationInfo {
  /// Two-letter [Alpha-2 ISO 3166-1](https://www.iso.org/iso-3166-country-codes.html)
  /// country code.
  pub country_code: String,
  /// Three-letter [Alpha-3 ISO 3166-1](https://www.iso.org/iso-3166-country-codes.html)
  /// country code.
  pub country_code3: String,
  /// Full country name in English.
  pub country_name: String,
  /// Full city name in English.
  pub city_name: String,
  /// Global latitude (horizontal) position.
  pub latitude: f32,
  /// Global longitude (vertical) position.
  pub longitude: f32,
  /// Time zone name as found in the [tz database](https://www.iana.org/time-zones).
  pub time_zone: String,
  /// Continent the country is located within. Can be AF (Africa), AN (Antarctica),
  /// AS (Asia), EU (Europe), NA (North America), OC (Oceania) or SA (South America).
  pub continent_code: String,
}

/// Retrieve log events for a specific user.
///
/// Note: For more information on all possible event types, their respective acronyms and
/// descriptions, see [Log Data Event Listing](https://auth0.com/docs/logs#log-data-event-listing).
///
/// For more information on the list of fields that can be used in `sort`, see
/// [Searchable Fields](https://auth0.com/docs/logs/query-syntax#searchable-fields).
///
/// Auth0 [limits the number of logs](https://auth0.com/docs/logs#limitations) you can
/// return by search criteria to 100 logs per request. Furthermore, you may only paginate
/// through up to 1,000 search results. If you exceed this threshold, please redefine your
/// search.
///
/// # Scopes
/// * `read:logs`
/// * `read:logs_users`
#[derive(Serialize)]
pub struct UserLogsGet<'a> {
  #[serde(skip_serializing)]
  client: &'a Auth0Client,

  #[serde(skip)]
  id: String,
  #[serde(flatten)]
  page: Page,
  #[serde(skip_serializing_if = "Sort::is_emtpy")]
  sort: Sort,
}

impl<'a> UserLogsGet<'a> {
  /// Create [GetUserLogs] request.
  pub fn new(client: &'a Auth0Client, id: &str) -> Self {
    Self {
      client,

      id: id.to_owned(),
      page: Default::default(),
      sort: Default::default(),
    }
  }
}

impl<'a> AsMut<Page> for UserLogsGet<'a> {
  fn as_mut(&mut self) -> &mut Page {
    &mut self.page
  }
}

impl<'a> AsMut<Sort> for UserLogsGet<'a> {
  fn as_mut(&mut self) -> &mut Sort {
    &mut self.sort
  }
}

impl<'a> AsRef<Auth0Client> for UserLogsGet<'a> {
  fn as_ref(&self) -> &Auth0Client {
    self.client
  }
}

impl<'a> Auth0RequestBuilder for UserLogsGet<'a> {
  type Response = Vec<UserLog>;

  fn build<F>(&self, factory: F) -> RequestBuilder
  where
    F: FnOnce(Method, &str) -> RequestBuilder,
  {
    factory(Method::GET, &format!("api/v2/users/{}/logs", self.id)).query(&self)
  }
}
