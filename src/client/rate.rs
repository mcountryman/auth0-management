use reqwest::Response;
use std::error::Error;
use std::time::{Duration, SystemTime};

pub struct RateLimit {
  last: SystemTime,
  limit: Option<u32>,
  reset: Option<SystemTime>,
  remaining: Option<u32>,
}

impl RateLimit {
  pub fn is_at_limit(&mut self) -> bool {
    if let Some(remaining) = self.remaining {
      if remaining > 0 {
        self.remaining = Some(remaining - 1);
        return true;
      } else {
      }
    }

    return true;
  }

  pub fn read(&mut self, res: &Response) {
    let headers = res.headers();

    self.limit = None;
    self.reset = None;
    self.remaining = None;

    if let Some(limit) = headers.get("x-rate-limit") {
      if let Ok(limit) = limit.to_str() {
        if let Ok(limit) = limit.parse::<u32>() {
          self.limit = Some(limit);
        }
      }
    }

    if let Some(remaining) = headers.get("x-rate-remaining") {
      if let Ok(remaining) = remaining.to_str() {
        if let Ok(remaining) = remaining.parse::<u32>() {
          self.remaining = Some(remaining);
        }
      }
    }

    if let Some(reset) = headers.get("x-rate-reset") {
      if let Ok(reset) = reset.to_str() {
        if let Ok(reset) = reset.parse::<u64>() {
          self.reset = Some(SystemTime::UNIX_EPOCH + Duration::from_secs(reset));
        }
      }
    }
  }
}
