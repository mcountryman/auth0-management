use serde::{Deserialize, Serialize};

use crate::helpers::get_client;
use auth0_management::{Auth0Request, Ordering, Pageable, Sortable, User};

mod helpers;

#[derive(Serialize, Deserialize, Debug)]
struct Metadata;

#[tokio::test]
async fn test_find_user() {
  let auth0 = get_client();

  // Create a user.
  auth0
    .create_user::<Metadata, Metadata>()
    .email("test@example.test")
    .connection("CONNECTION_ID")
    .send()
    .await
    .expect("Failed to create a user.");

  // Find first user user sort by email address.
  let users: Vec<User<Metadata, Metadata>> = auth0
    .find_users()
    .page(0)
    .per_page(1)
    .sort("email", Ordering::Ascending)
    .send()
    .await
    .expect("Failed to fetch users.");

  // Update found user.
  auth0
    .update_user::<Metadata, Metadata, &str>(
      &users.first().expect("No users found").user_id,
    )
    .email("test@test.test")
    .send()
    .await
    .expect("Failed to update user.");
}
