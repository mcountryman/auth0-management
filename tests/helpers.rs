use auth0_management::Auth0;

pub fn get_client() -> Auth0 {
  Auth0::builder()
    .domain(&env!("AUTH0_DOMAIN"))
    .audience(&env!("AUTH0_AUDIENCE"))
    .client_id(&env!("AUTH0_CLIENT_ID"))
    .client_secret(env!("AUTH0_CLIENT_SECRET"))
    .build()
    .unwrap()
}
