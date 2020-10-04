use std::env::var;
use std::time::{Duration, SystemTime};

use auth0_management::token::{Token, TokenManager, TokenManagerError};

/// Tests ClientTokenManagerBuilder and all the errors that can occur when parameters are not
/// supplied.
#[test]
fn test_build_manager() {
  get_working_client();

  assert!(TokenManager::new().build().is_err());

  assert_eq!(
    TokenManager::new()
      .domain(&var("AUTH0_DOMAIN").unwrap())
      .audience(&var("AUTH0_AUDIENCE").unwrap())
      .client_id(&var("AUTH0_CLIENT_ID").unwrap())
      .build()
      .err()
      .unwrap(),
    TokenManagerError::MissingClientSecret
  );

  assert_eq!(
    TokenManager::new()
      .domain(&var("AUTH0_DOMAIN").unwrap())
      .audience(&var("AUTH0_AUDIENCE").unwrap())
      .build()
      .err()
      .unwrap(),
    TokenManagerError::MissingClientID
  );

  assert_eq!(
    TokenManager::new()
      .domain(&var("AUTH0_DOMAIN").unwrap())
      .build()
      .err()
      .unwrap(),
    TokenManagerError::MissingAudience
  );

  assert_eq!(
    TokenManager::new().build().err().unwrap(),
    TokenManagerError::MissingDomain
  );
}

/// Basic fetch token test.
#[tokio::test]
async fn test_get_token() {
  let mut manager = get_working_client();

  // Fetches our first token.
  let token_a = manager.get_token().await.unwrap();
  // Fetch the second token. (should be cached)
  let token_b = manager.get_token().await.unwrap();

  // Ensure our token didn't expire to validate this came from Auth0.
  assert_eq!(token_a, token_b);
}

/// Hacky way of testing token expiration.
#[tokio::test]
async fn test_update_token() {
  let mut manager = get_working_client();

  // Mock "old" token with 0s expiration date.
  manager.token = Some(Token {
    expires_in: 0,
    access_token: "test".to_owned(),
  });

  // Mock token last fetch date time to now.
  manager.token_last = SystemTime::now();

  // Delay for 1s to simulate expiration of mocked token.
  tokio::time::delay_for(Duration::from_secs(1)).await;

  // Get new token.
  let token = manager.get_token().await.unwrap();

  // Ensure token isn't the mocked one we defined.
  assert_ne!(token, "test");
}

fn get_working_client() -> TokenManager {
  TokenManager::new()
    .domain(&var("AUTH0_DOMAIN").unwrap())
    .audience(&var("AUTH0_AUDIENCE").unwrap())
    .client_id(&var("AUTH0_CLIENT_ID").unwrap())
    .client_secret(&var("AUTH0_CLIENT_SECRET").unwrap())
    .build()
    .unwrap()
}
