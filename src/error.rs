//! Auth0 errors.

use crate::http::HttpRequestBuilderError;
use crate::entities::users::UserCreateBuilderError;
use thiserror::Error;

/// An Auth0 management result;
pub type Auth0Result<T> = Result<T, Auth0Error>;

/// An Auth0 management client error.
#[derive(Error, Debug)]
pub enum Auth0Error {
  /// A url parsing error.
  #[error(transparent)]
  Url(#[from] url::ParseError),
  /// A reqwest error.
  #[error(transparent)]
  Reqwest(#[from] reqwest::Error),

  /// A [HttpRequestBuilder] error.
  #[error(transparent)]
  HttpRequestBuilder(#[from] HttpRequestBuilderError),

  /// A [UserCreateBuilder] error.
  #[error(transparent)]
  UserCreateBuilder(#[from] UserCreateBuilderError),
}
