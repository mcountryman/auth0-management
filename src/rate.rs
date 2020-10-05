//! Provides ability to read rate limit headers and check if limits are exceeded.
//! # Examples
//! ```
//! use std::error::Error;
//! use reqwest::Client;
//! use auth0_management::rate::{RateLimit, RateLimitResponse};
//!
//! async fn request_thing(client: &Client, limit: &mut RateLimit) {
//!   if !limit.check_limit() {
//!     panic!("Rate limit exceeded!");
//!   }
//!
//!   client
//!     .get("https://example.com")
//!     .send()
//!     .await
//!     .unwrap()
//!     .rate_limit(limit);
//! }
//! ```
use std::error::Error;
use std::num::ParseIntError;
use std::time::{Duration, SystemTime};

use reqwest::header::ToStrError;
use reqwest::Response;
use serde::export::fmt::Display;
use serde::export::Formatter;

/// Provides ability to read rate limit headers and check if limits are exceeded.
pub struct RateLimit {
  pub limit: u32,
  pub reset: SystemTime,
  pub remaining: u32,
}

impl RateLimit {
  /// Gets instance of [RateLimit]
  pub fn new() -> Self {
    Self {
      limit: u32::MAX,
      reset: SystemTime::UNIX_EPOCH,
      remaining: 0,
    }
  }

  /// Gets boolean determining if rate limit was exceeded.
  pub fn check_limit(&mut self) -> bool {
    if SystemTime::now() >= self.reset {
      self.remaining = self.limit;
    }

    if self.remaining > 0 {
      self.remaining -= 1;

      true
    } else {
      false
    }
  }

  /// Read response headers and updates limits.
  pub fn read(&mut self, res: &Response) -> Result<(), RateLimitError> {
    let headers = res.headers();

    self.limit = headers
      .get("x-ratelimit-limit")
      .ok_or(RateLimitError::MissingRateLimitHeader)?
      .to_str()?
      .parse()?;

    self.remaining = headers
      .get("x-ratelimit-remaining")
      .ok_or(RateLimitError::MissingRateRemainingHeader)?
      .to_str()?
      .parse()?;

    self.reset = SystemTime::UNIX_EPOCH
      + Duration::from_secs(
        headers
          .get("x-ratelimit-reset")
          .ok_or(RateLimitError::MissingRateResetHeader)?
          .to_str()?
          .parse::<u64>()?,
      );

    Ok(())
  }
}

pub trait RateLimitResponse: Sized {
  fn rate_limit(
    self,
    rate_limit: &mut RateLimit,
  ) -> Result<Self, Box<dyn Error + Send + Sync>>;
}

impl RateLimitResponse for Response {
  fn rate_limit(
    self,
    rate_limit: &mut RateLimit,
  ) -> Result<Self, Box<dyn Error + Send + Sync>> {
    rate_limit.read(&self)?;
    Ok(self)
  }
}

impl Default for RateLimit {
  fn default() -> Self {
    RateLimit::new()
  }
}

#[derive(Debug)]
pub enum RateLimitError {
  MissingRateLimitHeader,
  MissingRateResetHeader,
  MissingRateRemainingHeader,

  BadHeaderEncoding(ToStrError),
  BadHeaderFormat(ParseIntError),
}

impl From<ToStrError> for RateLimitError {
  fn from(err: ToStrError) -> Self {
    RateLimitError::BadHeaderEncoding(err)
  }
}

impl From<ParseIntError> for RateLimitError {
  fn from(err: ParseIntError) -> Self {
    RateLimitError::BadHeaderFormat(err)
  }
}

impl Display for RateLimitError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for RateLimitError {}
