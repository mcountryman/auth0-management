//! Auth0 errors.

use crate::entities::users::UserCreateBuilderError;
use crate::http::HttpRequestBuilderError;
use std::convert::Infallible;
use thiserror::Error;

/// An Auth0 management result;
pub type Auth0Result<T> = Result<T, Auth0Error>;

/// An Auth0 management client error.
#[derive(Error, Debug)]
pub enum Auth0Error {
  /// An infallible error.
  #[error("An Infallible error occurred.")]
  Infallible(#[from] Infallible),

  /// A url parsing error.
  #[error(transparent)]
  Url(#[from] url::ParseError),
  /// A reqwest error.
  #[error(transparent)]
  Reqwest(#[from] reqwest::Error),
  /// A reqwest-middleware error.
  #[error(transparent)]
  ReqwestMiddleware(#[from] reqwest_middleware::Error),

  /// A [HttpRequestBuilder] error.
  #[error(transparent)]
  HttpRequestBuilder(#[from] HttpRequestBuilderError),

  /// A [UserCreateBuilder] error.
  #[error(transparent)]
  UserCreateBuilder(#[from] UserCreateBuilderError),
}
