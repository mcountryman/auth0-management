use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use reqwest::header::ToStrError;
use reqwest::Response;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// Provides ability to read rate limit headers and check if limits are exceeded.
#[derive(Debug)]
pub struct RateLimit {
  limit: AtomicU32,
  reset: AtomicU64,
  remaining: AtomicU32,
}

impl RateLimit {
  /// Gets instance of [RateLimit]
  pub fn new() -> Self {
    Self {
      limit: AtomicU32::new(u32::MAX),
      reset: AtomicU64::new(0),
      remaining: AtomicU32::new(0),
    }
  }

  /// Gets boolean determining if rate limit was exceeded.
  pub fn check_limit(&self) -> bool {
    let now = SystemTime::now();
    let expire = UNIX_EPOCH + Duration::from_secs(self.reset.load(Ordering::Relaxed));

    if now >= expire {
      self
        .remaining
        .store(self.limit.load(Ordering::SeqCst), Ordering::SeqCst);
    }

    // TODO: Make this thread safe.
    if self.remaining.load(Ordering::SeqCst) > 0 {
      self.remaining.fetch_sub(1, Ordering::SeqCst);

      true
    } else {
      false
    }
  }

  /// Read response headers and updates limits.
  pub fn read(&self, res: &Response) -> Result<(), RateLimitError> {
    let headers = res.headers();

    self.limit.store(
      headers
        .get("x-ratelimit-limit")
        .ok_or(RateLimitError::MissingRateLimitHeader)?
        .to_str()?
        .parse()?,
      Ordering::SeqCst,
    );

    self.remaining.store(
      headers
        .get("x-ratelimit-remaining")
        .ok_or(RateLimitError::MissingRateRemainingHeader)?
        .to_str()?
        .parse()?,
      Ordering::SeqCst,
    );

    self.reset.store(
      headers
        .get("x-ratelimit-reset")
        .ok_or(RateLimitError::MissingRateResetHeader)?
        .to_str()?
        .parse::<u64>()?,
      Ordering::SeqCst,
    );

    Ok(())
  }
}

pub trait RateLimitResponse: Sized {
  fn rate_limit(self, rate_limit: &RateLimit) -> Result<Self, RateLimitError>;
}

impl RateLimitResponse for Response {
  fn rate_limit(self, rate_limit: &RateLimit) -> Result<Self, RateLimitError> {
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
