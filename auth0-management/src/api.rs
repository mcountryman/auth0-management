mod stats {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetDailyResponse {
    /// Number of logins on this date.
    logins: i32,
    /// Date these events occurred in ISO 8601 format.
    date: String,
    /// Number of breached-password detections on this date (subscription required).
    leaked_passwords: i32,
    /// Date and time this stats entry was last updated in ISO 8601 format.
    updated_at: String,
    /// Approximate date and time the first event occurred in ISO 8601 format.
    created_at: String,
    /// Number of signups on this date.
    signups: i32,
  }
}

mod tickets {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostEmailVerificationBodyIdentity {
    /// Identity provider name of the identity (e.g. `google-oauth2`).
    provider: String,
    /// user_id of the identity to be verified.
    user_id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostEmailVerificationResponse {
    /// URL representing the ticket.
    ticket: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostPasswordChangeResponse {
    /// URL representing the ticket.
    ticket: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostPasswordChangeBody {
    /// URL the user will be redirected to once ticket is used.
    result_url: String,
    /// user_id of for whom the ticket should be created.
    user_id: String,
    /// Email address of the user for whom the tickets should be created. Requires the connection_id parameter. Cannot be specified when using user_id.
    email: String,
    /// ID of the connection. If provided, allows the user to be specified using email instead of user_id. If you set this value, you must also send the email parameter. You cannot send user_id when specifying a connection_id.
    connection_id: String,
    /// Number of seconds for which the ticket is valid before expiration. If unspecified or set to 0, this value defaults to 432000 seconds (5 days).
    ttl_sec: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostEmailVerificationBody {
    /// user_id of for whom the ticket should be created.
    user_id: String,
    /// URL the user will be redirected to once ticket is used.
    result_url: String,
    /// This must be provided to verify primary social, enterprise and passwordless email identities. Also, is needed to verify secondary identities.
    identity: PostEmailVerificationBodyIdentity,
    /// Number of seconds for which the ticket is valid before expiration. If unspecified or set to 0, this value defaults to 432000 seconds (5 days).
    ttl_sec: i32,
  }
}

mod anomaly {
  use serde::{Deserialize, Serialize};
}

mod custom_domains {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostCustomDomainsResponseVerification;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostCustomDomainsBody {
    /// Custom domain verification method. Must be `txt`.
    verification_method: String,
    /// Domain name.
    domain: String,
    /// compatible includes TLS 1.0, 1.1, 1.2, and recommended only includes TLS 1.2
    tls_policy: String,
    /// HTTP header to fetch client IP header. Ex: CF-Connecting-IP, X-Forwarded-For or True-Client-IP.
    custom_client_ip_header: String,
    /// Custom domain provisioning type. Must be `auth0_managed_certs` or `self_managed_certs`.
    #[serde(rename = "type")]
    kind: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetCustomDomainsResponseVerification;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchCustomDomainsByIdBody {
    /// compatible includes TLS 1.0, 1.1, 1.2, and recommended only includes TLS 1.2
    tls_policy: String,
    /// The HTTP header to fetch the client's IP address
    custom_client_ip_header: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostCustomDomainsResponse {
    /// Custom domain configuration status. Can be `disabled`, `pending`, `pending_verification`, or `ready`.
    status: String,
    /// The HTTP header to fetch the client's IP address
    custom_client_ip_header: String,
    /// ID of the custom domain.
    custom_domain_id: String,
    /// Domain verification settings.
    verification: PostCustomDomainsResponseVerification,
    /// Custom domain provisioning type. Can be `auth0_managed_certs` or `self_managed_certs`.
    #[serde(rename = "type")]
    kind: String,
    /// Domain name.
    domain: String,
    /// The TLS version policy
    tls_policy: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchCustomDomainsByIdResponse {
    /// The HTTP header to fetch the client's IP address
    custom_client_ip_header: String,
    /// Domain verification settings.
    verification: PatchCustomDomainsByIdResponseVerification,
    /// ID of the custom domain.
    custom_domain_id: String,
    /// Domain name.
    domain: String,
    /// Custom domain provisioning type. Can be `auth0_managed_certs` or `self_managed_certs`.
    #[serde(rename = "type")]
    kind: String,
    /// The TLS version policy
    tls_policy: String,
    /// Custom domain configuration status. Can be `disabled`, `pending`, `pending_verification`, or `ready`.
    status: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchCustomDomainsByIdResponseVerification;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostVerifyResponse {
    /// Custom domain configuration status. Can be `disabled`, `pending`, `pending_verification`, or `ready`.
    status: String,
    /// Intermediate address.
    origin_domain_name: String,
    /// ID of the custom domain.
    custom_domain_id: String,
    /// Custom domain provisioning type. Can be `auth0_managed_certs` or `self_managed_certs`.
    #[serde(rename = "type")]
    kind: String,
    /// Domain verification settings.
    verification: PostVerifyResponseVerification,
    /// The HTTP header to fetch the client's IP address
    custom_client_ip_header: String,
    /// Domain name.
    domain: String,
    /// CNAME API key header.
    cname_api_key: String,
    /// The TLS version policy
    tls_policy: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostVerifyResponseVerification;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetCustomDomainsResponse {
    /// Intermediate address.
    origin_domain_name: String,
    /// The HTTP header to fetch the client's IP address
    custom_client_ip_header: String,
    /// Domain verification settings.
    verification: GetCustomDomainsResponseVerification,
    /// The TLS version policy
    tls_policy: String,
    /// ID of the custom domain.
    custom_domain_id: String,
    /// Domain name.
    domain: String,
    /// Custom domain provisioning type. Can be `auth0_managed_certs` or `self_managed_certs`.
    #[serde(rename = "type")]
    kind: String,
    /// Custom domain configuration status. Can be `disabled`, `pending`, `pending_verification`, or `ready`.
    status: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetCustomDomainsByIdResponse {
    /// Domain name.
    domain: String,
    /// The TLS version policy
    tls_policy: String,
    /// The HTTP header to fetch the client's IP address
    custom_client_ip_header: String,
    /// Domain verification settings.
    verification: GetCustomDomainsByIdResponseVerification,
    /// ID of the custom domain.
    custom_domain_id: String,
    /// Custom domain configuration status. Can be `disabled`, `pending`, `pending_verification`, or `ready`.
    status: String,
    /// Custom domain provisioning type. Can be `auth0_managed_certs` or `self_managed_certs`.
    #[serde(rename = "type")]
    kind: String,
    /// Intermediate address.
    origin_domain_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetCustomDomainsByIdResponseVerification;
}

mod guardian {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PutFactorsByNameResponse;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchSnsResponse {
    aws_access_key_id: String,
    sns_apns_platform_application_arn: String,
    aws_secret_access_key: String,
    sns_gcm_platform_application_arn: String,
    aws_region: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PutFactorsByNameBody;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetPoliciesResponse;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PutMessageTypesResponse;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetTemplatesResponse {
    /// Message sent to the user when they are invited to enroll with a phone number.
    enrollment_message: String,
    /// Message sent to the user when they are prompted to verify their account.
    verification_message: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetTwilioResponse {
    /// Twilio Authentication token
    auth_token: String,
    /// Twilio SID
    sid: String,
    /// Copilot SID
    messaging_service_sid: String,
    /// From number
    from: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PutPoliciesResponse;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PutSnsBody {
    sns_apns_platform_application_arn: String,
    aws_region: String,
    sns_gcm_platform_application_arn: String,
    aws_secret_access_key: String,
    aws_access_key_id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PutSnsResponse {
    aws_region: String,
    sns_gcm_platform_application_arn: String,
    aws_access_key_id: String,
    sns_apns_platform_application_arn: String,
    aws_secret_access_key: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetFactorsResponse {
    /// Factor name. Can be `sms`, `push-notification`, `email`, `duo` or `otp`.
    name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetSelectedProviderResponse {
    provider: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PutSelectedProviderResponse {
    provider: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetMessageTypesResponse;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PutTwilioResponse {
    /// From number
    from: String,
    /// Copilot SID
    messaging_service_sid: String,
    /// Twilio SID
    sid: String,
    /// Twilio Authentication token
    auth_token: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PutMessageTypesBody;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PutTwilioBody {
    /// Copilot SID
    messaging_service_sid: String,
    /// From number
    from: String,
    /// Twilio SID
    sid: String,
    /// Twilio Authentication token
    auth_token: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetSnsResponse {
    sns_gcm_platform_application_arn: String,
    sns_apns_platform_application_arn: String,
    aws_region: String,
    aws_secret_access_key: String,
    aws_access_key_id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetEnrollmentsByIdResponse {
    /// Phone number.
    phone_number: String,
    /// Enrollment date and time.
    enrolled_at: String,
    /// Status of this enrollment. Can be `pending` or `confirmed`.
    status: String,
    /// Device name (only for push notification).
    name: String,
    /// Last authentication date and time.
    last_auth: String,
    /// ID for this enrollment.
    id: String,
    /// Device identifier. This is usually the phone identifier.
    identifier: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PutSelectedProviderBody {
    provider: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchSnsBody {
    sns_gcm_platform_application_arn: String,
    aws_region: String,
    aws_secret_access_key: String,
    aws_access_key_id: String,
    sns_apns_platform_application_arn: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PutTemplatesResponse {
    /// Message sent to the user when they are invited to enroll with a phone number.
    enrollment_message: String,
    /// Message sent to the user when they are prompted to verify their account.
    verification_message: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PutPoliciesBody;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PutTemplatesBody {
    /// Message sent to the user when they are invited to enroll with a phone number.
    enrollment_message: String,
    /// Message sent to the user when they are prompted to verify their account.
    verification_message: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostTicketResponse {
    /// The url you can use to start enrollment
    ticket_url: String,
    /// The ticket_id used to identify the enrollment
    ticket_id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostTicketBody {
    /// user_id for the enrollment ticket
    user_id: String,
    /// alternate email to which the enrollment email will be sent. Optional - by default, the email will be sent to the user's default address
    email: String,
  }
}

mod user_blocks {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetUserBlocksResponse;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetUserBlocksByIdResponse;
}

mod hooks {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostHooksResponseDependencies;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostSecretsResponse;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetHooksResponse {
    /// Name of this hook.
    name: String,
    /// Code to be executed when this hook runs.
    script: String,
    /// Trigger ID
    #[serde(rename = "triggerId")]
    trigger_id: String,
    /// Dependencies of this hook used by webtask server.
    dependencies: GetHooksResponseDependencies,
    /// ID of this hook.
    id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetSecretsResponse;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchSecretsResponse;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchSecretsBody;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostHooksBodyDependencies;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostHooksBody {
    /// Dependencies of this hook used by webtask server.
    dependencies: PostHooksBodyDependencies,
    /// Execution stage of this rule. Can be `credentials-exchange`, `pre-user-registration`, `post-user-registration`, `post-change-password`, or `send-phone-message`.
    #[serde(rename = "triggerId")]
    trigger_id: String,
    /// Name of this hook.
    name: String,
    /// Code to be executed when this hook runs.
    script: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct DeleteSecretsBody;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchHooksByIdResponseDependencies;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchHooksByIdResponse {
    /// ID of this hook.
    id: String,
    /// Name of this hook.
    name: String,
    /// Trigger ID
    #[serde(rename = "triggerId")]
    trigger_id: String,
    /// Code to be executed when this hook runs.
    script: String,
    /// Dependencies of this hook used by webtask server.
    dependencies: PatchHooksByIdResponseDependencies,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostHooksResponse {
    /// Trigger ID
    #[serde(rename = "triggerId")]
    trigger_id: String,
    /// Code to be executed when this hook runs.
    script: String,
    /// Dependencies of this hook used by webtask server.
    dependencies: PostHooksResponseDependencies,
    /// ID of this hook.
    id: String,
    /// Name of this hook.
    name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostSecretsBody;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchHooksByIdBody {
    /// Name of this hook.
    name: String,
    /// Code to be executed when this hook runs.
    script: String,
    /// Dependencies of this hook used by webtask server.
    dependencies: PatchHooksByIdBodyDependencies,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchHooksByIdBodyDependencies;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetHooksByIdResponse {
    /// ID of this hook.
    id: String,
    /// Dependencies of this hook used by webtask server.
    dependencies: GetHooksByIdResponseDependencies,
    /// Trigger ID
    #[serde(rename = "triggerId")]
    trigger_id: String,
    /// Name of this hook.
    name: String,
    /// Code to be executed when this hook runs.
    script: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetHooksByIdResponseDependencies;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetHooksResponseDependencies;
}

mod device_credentials {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostDeviceCredentialsBody {
    /// Type of credential. Must be `public_key`.
    #[serde(rename = "type")]
    kind: String,
    /// Name for this device easily recognized by owner.
    device_name: String,
    /// Unique identifier for the device. Recommend using <a href="http://developer.android.com/reference/android/provider/Settings.Secure.html#ANDROID_ID">Android_ID</a> on Android and <a href="https://developer.apple.com/library/ios/documentation/UIKit/Reference/UIDevice_Class/index.html#//apple_ref/occ/instp/UIDevice/identifierForVendor">identifierForVendor</a>.
    device_id: String,
    /// client_id of the client (application) this credential is for.
    client_id: String,
    /// Base64 encoded string containing the credential.
    value: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetDeviceCredentialsResponse {
    /// Type of credential. Can be `public_key` or `refresh_token`.
    #[serde(rename = "type")]
    kind: String,
    /// user_id this credential is associated with.
    user_id: String,
    /// Unique identifier for the device. Recommend using <a href="http://developer.android.com/reference/android/provider/Settings.Secure.html#ANDROID_ID">Android_ID</a> on Android and <a href="https://developer.apple.com/library/ios/documentation/UIKit/Reference/UIDevice_Class/index.html#//apple_ref/occ/instp/UIDevice/identifierForVendor">identifierForVendor</a>.
    device_id: String,
    /// ID of this device.
    id: String,
    /// client_id of the client (application) this credential is for.
    client_id: String,
    /// Name for this device easily recognized by owner.
    device_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostDeviceCredentialsResponse {
    /// The credential's identifier
    id: String,
  }
}

mod grants {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetGrantsResponse {
    /// ID of the user.
    user_id: String,
    /// ID of the client.
    #[serde(rename = "clientID")]
    client_id: String,
    /// Audience of the grant.
    audience: String,
    /// ID of the grant.
    id: String,
  }
}

mod client_grants {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientGrantsResponse {
    /// ID of the client grant.
    id: String,
    /// ID of the client.
    client_id: String,
    /// Audience or API identifier of this client grant.
    audience: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientGrantsBody {
    /// Audience or API identifier of this client grant.
    audience: String,
    /// ID of the client.
    client_id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientGrantsByIdResponse {
    /// Audience or API identifier of this client grant.
    audience: String,
    /// ID of the client grant.
    id: String,
    /// ID of the client.
    client_id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientGrantsByIdBody;
}

mod users {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetUsersByIdResponse {
    /// User metadata to which this user has read-only access.
    app_metadata: GetUsersByIdResponseAppMetadata,
    /// Date and time when this user was created (ISO_8601 format).
    created_at: String,
    /// Username of this user.
    username: String,
    /// Date and time when this user was last updated/modified (ISO_8601 format).
    updated_at: String,
    /// Name of this user.
    name: String,
    /// Given name/first name/forename of this user.
    given_name: String,
    /// User metadata to which this user has read/write access.
    user_metadata: GetUsersByIdResponseUserMetadata,
    /// ID of the user which can be used when interacting with other APIs.
    user_id: String,
    /// Last IP address from which this user logged in.
    last_ip: String,
    /// Total number of logins this user has performed.
    logins_count: i32,
    /// Family name/last name/surname of this user.
    family_name: String,
    /// Preferred nickname or alias of this user.
    nickname: String,
    /// Last date and time this user logged in (ISO_8601 format).
    last_login: String,
    /// Email address of this user.
    email: String,
    /// Phone number for this user when using SMS connections. Follows the <a href="https://en.wikipedia.org/wiki/E.164">E.164 recommendation</a>.
    phone_number: String,
    /// URL to picture, photo, or avatar of this user.
    picture: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostIdentitiesResponse {
    /// IDP access token returned if scope `read:user_idp_token` is defined.
    access_token: String,
    user_id: String,
    #[serde(rename = "profileData")]
    profile_data: PostIdentitiesResponseProfileData,
    /// Type of identity provider.
    provider: String,
    /// Connection name of this identity.
    connection: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostUsersBody {
    /// Data related to the user that does not affect the application's core functionality.
    user_metadata: PostUsersBodyUserMetadata,
    /// Data related to the user that does affect the application's core functionality.
    app_metadata: PostUsersBodyAppMetadata,
    /// The user's nickname.
    nickname: String,
    /// The user's username. Only valid if the connection requires a username.
    username: String,
    /// The user's email.
    email: String,
    /// The user's family name(s).
    family_name: String,
    /// A URI pointing to the user's picture.
    picture: String,
    /// The external user's id provided by the identity provider.
    user_id: String,
    /// The user's given name(s).
    given_name: String,
    /// Name of the connection this user should be created in.
    connection: String,
    /// The user's full name.
    name: String,
    /// Initial password for this user (mandatory for non-SMS connections).
    password: String,
    /// The user's phone number (following the E.164 recommendation), only valid for users from SMS connections.
    phone_number: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostPermissionsBody;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostUserRolesBody;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostUsersResponseAppMetadata;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct DeletePermissionsBody;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct DeleteUserIdentityByUserIdResponse {
    /// IDP access token returned only if scope read:user_idp_token is defined
    access_token: String,
    #[serde(rename = "profileData")]
    profile_data: DeleteUserIdentityByUserIdResponseProfileData,
    /// The name of the connection for the identity.
    connection: String,
    /// The unique identifier for the user for the identity.
    user_id: String,
    /// The type of identity provider.
    provider: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct DeleteUserIdentityByUserIdResponseProfileData {
    /// Phone number for this user.
    phone_number: String,
    /// Family name/last name/surname of this user.
    family_name: String,
    /// Username of this user.
    username: String,
    /// Email address of this user.
    email: String,
    /// Name of this user.
    name: String,
    /// Given name/first name/forename of this user.
    given_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetLogsByUserResponseDetails;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchUsersByIdBodyUserMetadata;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostUsersResponseUserMetadata;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchUsersByIdBody {
    /// Family name/last name/surname of this user.
    family_name: String,
    /// User metadata to which this user has read/write access.
    user_metadata: PatchUsersByIdBodyUserMetadata,
    /// User metadata to which this user has read-only access.
    app_metadata: PatchUsersByIdBodyAppMetadata,
    /// Preferred nickname or alias of this user.
    nickname: String,
    /// The user's username. Only valid if the connection requires a username.
    username: String,
    /// Given name/first name/forename of this user.
    given_name: String,
    /// Email address of this user.
    email: String,
    /// The user's phone number (following the E.164 recommendation), only valid for users from SMS connections.
    phone_number: String,
    /// New password for this user (mandatory for non-SMS connections).
    password: String,
    /// ID of the connection this user should be created in.
    connection: String,
    /// Name of this user.
    name: String,
    /// Auth0 client ID. Only valid when updating email address.
    client_id: String,
    /// URL to picture, photo, or avatar of this user.
    picture: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchUsersByIdBodyAppMetadata;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchUsersByIdResponseUserMetadata;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostIdentitiesResponseProfileData {
    /// Phone number for this user.
    phone_number: String,
    /// Family name/last name/surname of this user.
    family_name: String,
    /// Username of this user.
    username: String,
    /// Email address of this user.
    email: String,
    /// Name of this user.
    name: String,
    /// Given name/first name/forename of this user.
    given_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetUsersResponseUserMetadata;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetEnrollmentsResponse {
    /// Authentication method for this enrollment. Can be `authentication`, `guardian`, or `sms`.
    auth_method: String,
    /// Name of enrollment (usually phone number).
    name: String,
    /// ID of this enrollment.
    id: String,
    /// Device identifier (usually phone identifier) of this enrollment.
    identifier: String,
    /// Last authentication date and time of this enrollment.
    last_auth: String,
    /// Status of this enrollment. Can be `pending` or `confirmed`.
    status: String,
    /// Type of enrollment.
    #[serde(rename = "type")]
    kind: String,
    /// Start date and time of this enrollment.
    enrolled_at: String,
    /// Phone number for this enrollment.
    phone_number: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetUsersResponseAppMetadata;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchUsersByIdResponse {
    /// Name of this user.
    name: String,
    /// Given name/first name/forename of this user.
    given_name: String,
    /// Date and time when this user was last updated/modified (ISO_8601 format).
    updated_at: String,
    /// User metadata to which this user has read-only access.
    app_metadata: PatchUsersByIdResponseAppMetadata,
    /// Preferred nickname or alias of this user.
    nickname: String,
    /// Username of this user.
    username: String,
    /// ID of the user which can be used when interacting with other APIs.
    user_id: String,
    /// User metadata to which this user has read/write access.
    user_metadata: PatchUsersByIdResponseUserMetadata,
    /// Date and time when this user was created (ISO_8601 format).
    created_at: String,
    /// Last IP address from which this user logged in.
    last_ip: String,
    /// Last date and time this user logged in (ISO_8601 format).
    last_login: String,
    /// Phone number for this user when using SMS connections. Follows the <a href="https://en.wikipedia.org/wiki/E.164">E.164 recommendation</a>.
    phone_number: String,
    /// Total number of logins this user has performed.
    logins_count: i32,
    /// Family name/last name/surname of this user.
    family_name: String,
    /// URL to picture, photo, or avatar of this user.
    picture: String,
    /// Email address of this user.
    email: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetUsersByIdResponseUserMetadata;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct DeleteUserRolesBody;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostIdentitiesBody {
    /// Identity provider of the secondary user account being linked.
    provider: String,
    /// connection_id of the secondary user account being linked when more than one `auth0` database provider exists.
    connection_id: String,
    /// JWT for the secondary account being linked. If sending this parameter, `provider`, `user_id`, and `connection_id` must not be sent.
    link_with: String,
    user_id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostUsersBodyAppMetadata;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetUsersResponse {
    /// Email address of this user.
    email: String,
    /// Total number of logins this user has performed.
    logins_count: i32,
    /// Given name/first name/forename of this user.
    given_name: String,
    /// User metadata to which this user has read-only access.
    app_metadata: GetUsersResponseAppMetadata,
    /// Last date and time this user logged in (ISO_8601 format).
    last_login: String,
    /// URL to picture, photo, or avatar of this user.
    picture: String,
    /// Last IP address from which this user logged in.
    last_ip: String,
    /// Date and time when this user was created (ISO_8601 format).
    created_at: String,
    /// User metadata to which this user has read/write access.
    user_metadata: GetUsersResponseUserMetadata,
    /// ID of the user which can be used when interacting with other APIs.
    user_id: String,
    /// Date and time when this user was last updated/modified (ISO_8601 format).
    updated_at: String,
    /// Preferred nickname or alias of this user.
    nickname: String,
    /// Family name/last name/surname of this user.
    family_name: String,
    /// Phone number for this user when using SMS connections. Follows the <a href="https://en.wikipedia.org/wiki/E.164">E.164 recommendation</a>.
    phone_number: String,
    /// Username of this user.
    username: String,
    /// Name of this user.
    name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostUsersResponse {
    /// Family name/last name/surname of this user.
    family_name: String,
    /// Preferred nickname or alias of this user.
    nickname: String,
    /// Date and time when this user was last updated/modified (ISO_8601 format).
    updated_at: String,
    /// User metadata to which this user has read/write access.
    user_metadata: PostUsersResponseUserMetadata,
    /// User metadata to which this user has read-only access.
    app_metadata: PostUsersResponseAppMetadata,
    /// Last IP address from which this user logged in.
    last_ip: String,
    /// Total number of logins this user has performed.
    logins_count: i32,
    /// Last date and time this user logged in (ISO_8601 format).
    last_login: String,
    /// Email address of this user.
    email: String,
    /// Date and time when this user was created (ISO_8601 format).
    created_at: String,
    /// Name of this user.
    name: String,
    /// URL to picture, photo, or avatar of this user.
    picture: String,
    /// ID of the user which can be used when interacting with other APIs.
    user_id: String,
    /// Username of this user.
    username: String,
    /// Phone number for this user when using SMS connections. Follows the <a href="https://en.wikipedia.org/wiki/E.164">E.164 recommendation</a>.
    phone_number: String,
    /// Given name/first name/forename of this user.
    given_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetUsersByIdResponseAppMetadata;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetLogsByUserResponse {
    /// Unique ID of the event.
    log_id: String,
    /// API audience the event applies to.
    audience: String,
    /// ID of the client (application).
    client_id: String,
    /// Description of this event.
    description: String,
    /// Name of the connection the event relates to.
    connection: String,
    /// Name of the strategy involved in the event.
    strategy: String,
    /// Date when the event occurred in ISO 8601 format.
    date: String,
    /// Name of the client (application).
    client_name: String,
    /// Name of the user involved in the event.
    user_name: String,
    /// ID of the connection the event relates to.
    connection_id: String,
    /// Scope permissions applied to the event.
    scope: String,
    /// Type of strategy involved in the event.
    strategy_type: String,
    /// User agent string from the client device that caused the event.
    user_agent: String,
    /// Information about the location that triggered this event based on the `ip`.
    location_info: GetLogsByUserResponseLocationInfo,
    /// Hostname the event applies to.
    hostname: String,
    /// IP address of the log event source.
    ip: String,
    /// ID of the user involved in the event.
    user_id: String,
    /// Additional useful details about this event (structure is dependent upon event type).
    details: GetLogsByUserResponseDetails,
    /// Type of event.
    #[serde(rename = "type")]
    kind: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetUserRolesResponse {
    /// Description of the role.
    description: String,
    /// Name of the role.
    name: String,
    /// ID for this role.
    id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetPermissionsResponse {
    /// Resource server (API) identifier that this permission is for.
    resource_server_identifier: String,
    /// Description of this permission.
    description: String,
    /// Resource server (API) name this permission is for.
    resource_server_name: String,
    /// Name of this permission.
    permission_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchUsersByIdResponseAppMetadata;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRecoveryCodeRegenerationResponse {
    /// New account recovery code.
    recovery_code: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetLogsByUserResponseLocationInfo {
    /// Full city name in English.
    city_name: String,
    /// Continent the country is located within. Can be `AF` (Africa), `AN` (Antarctica), `AS` (Asia), `EU` (Europe), `NA` (North America), `OC` (Oceania) or `SA` (South America).
    continent_code: String,
    /// Time zone name as found in the <a href="https://www.iana.org/time-zones">tz database</a>.
    time_zone: String,
    /// Global latitude (horizontal) position.
    latitude: String,
    /// Three-letter <a href="https://www.iso.org/iso-3166-country-codes.html">Alpha-3 ISO 3166-1</a> country code.
    #[serde(rename = "country_code3")]
    country_code_3: String,
    /// Two-letter <a href="https://www.iso.org/iso-3166-country-codes.html">Alpha-2 ISO 3166-1</a> country code.
    country_code: String,
    /// Full country name in English.
    country_name: String,
    /// Global longitude (vertical) position.
    longitude: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostUsersBodyUserMetadata;
}

mod users_by_email {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetUsersByEmailResponseUserMetadata;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetUsersByEmailResponse {
    /// Username of this user.
    username: String,
    /// Preferred nickname or alias of this user.
    nickname: String,
    /// Last date and time this user logged in (ISO_8601 format).
    last_login: String,
    /// Name of this user.
    name: String,
    /// Family name/last name/surname of this user.
    family_name: String,
    /// Date and time when this user was created (ISO_8601 format).
    created_at: String,
    /// User metadata to which this user has read-only access.
    app_metadata: GetUsersByEmailResponseAppMetadata,
    /// User metadata to which this user has read/write access.
    user_metadata: GetUsersByEmailResponseUserMetadata,
    /// URL to picture, photo, or avatar of this user.
    picture: String,
    /// Phone number for this user when using SMS connections. Follows the <a href="https://en.wikipedia.org/wiki/E.164">E.164 recommendation</a>.
    phone_number: String,
    /// Total number of logins this user has performed.
    logins_count: i32,
    /// ID of the user which can be used when interacting with other APIs.
    user_id: String,
    /// Given name/first name/forename of this user.
    given_name: String,
    /// Email address of this user.
    email: String,
    /// Date and time when this user was last updated/modified (ISO_8601 format).
    updated_at: String,
    /// Last IP address from which this user logged in.
    last_ip: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetUsersByEmailResponseAppMetadata;
}

mod email_templates {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchEmailTemplatesByTemplateNameResponse {
    /// Subject line of the email.
    subject: String,
    /// Syntax of the template body.
    syntax: String,
    /// Senders `from` email address.
    from: String,
    /// URL to redirect the user to after a successful action.
    #[serde(rename = "resultUrl")]
    result_url: String,
    /// Template name. Can be `verify_email`, `verify_email_by_code`, `reset_email`, `welcome_email`, `blocked_account`, `stolen_credentials`, `enrollment_email`, `mfa_oob_code`, `change_password` (legacy), or `password_reset` (legacy).
    template: String,
    /// Body of the email template.
    body: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostEmailTemplatesResponse {
    /// Subject line of the email.
    subject: String,
    /// Template name. Can be `verify_email`, `verify_email_by_code`, `reset_email`, `welcome_email`, `blocked_account`, `stolen_credentials`, `enrollment_email`, `mfa_oob_code`, `change_password` (legacy), or `password_reset` (legacy).
    template: String,
    /// URL to redirect the user to after a successful action.
    #[serde(rename = "resultUrl")]
    result_url: String,
    /// Syntax of the template body.
    syntax: String,
    /// Senders `from` email address.
    from: String,
    /// Body of the email template.
    body: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchEmailTemplatesByTemplateNameBody {
    /// Template name. Can be `verify_email`, `verify_email_by_code`, `reset_email`, `welcome_email`, `blocked_account`, `stolen_credentials`, `enrollment_email`, `mfa_oob_code`, `change_password` (legacy), or `password_reset` (legacy).
    template: String,
    /// Body of the email template.
    body: String,
    /// Senders `from` email address.
    from: String,
    /// Subject line of the email.
    subject: String,
    /// Syntax of the template body.
    syntax: String,
    /// URL to redirect the user to after a successful action.
    #[serde(rename = "resultUrl")]
    result_url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PutEmailTemplatesByTemplateNameResponse {
    /// Body of the email template.
    body: String,
    /// Senders `from` email address.
    from: String,
    /// Syntax of the template body.
    syntax: String,
    /// URL to redirect the user to after a successful action.
    #[serde(rename = "resultUrl")]
    result_url: String,
    /// Subject line of the email.
    subject: String,
    /// Template name. Can be `verify_email`, `verify_email_by_code`, `reset_email`, `welcome_email`, `blocked_account`, `stolen_credentials`, `enrollment_email`, `mfa_oob_code`, `change_password` (legacy), or `password_reset` (legacy).
    template: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetEmailTemplatesByTemplateNameResponse {
    /// URL to redirect the user to after a successful action.
    #[serde(rename = "resultUrl")]
    result_url: String,
    /// Senders `from` email address.
    from: String,
    /// Syntax of the template body.
    syntax: String,
    /// Subject line of the email.
    subject: String,
    /// Body of the email template.
    body: String,
    /// Template name. Can be `verify_email`, `verify_email_by_code`, `reset_email`, `welcome_email`, `blocked_account`, `stolen_credentials`, `enrollment_email`, `mfa_oob_code`, `change_password` (legacy), or `password_reset` (legacy).
    template: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostEmailTemplatesBody {
    /// Body of the email template.
    body: String,
    /// Subject line of the email.
    subject: String,
    /// Senders `from` email address.
    from: String,
    /// Syntax of the template body.
    syntax: String,
    /// Template name. Can be `verify_email`, `verify_email_by_code`, `reset_email`, `welcome_email`, `blocked_account`, `stolen_credentials`, `enrollment_email`, `mfa_oob_code`, `change_password` (legacy), or `password_reset` (legacy).
    template: String,
    /// URL to redirect the user to after a successful action.
    #[serde(rename = "resultUrl")]
    result_url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PutEmailTemplatesByTemplateNameBody {
    /// Syntax of the template body.
    syntax: String,
    /// Template name. Can be `verify_email`, `verify_email_by_code`, `reset_email`, `welcome_email`, `blocked_account`, `stolen_credentials`, `enrollment_email`, `mfa_oob_code`, `change_password` (legacy), or `password_reset` (legacy).
    template: String,
    /// Body of the email template.
    body: String,
    /// Subject line of the email.
    subject: String,
    /// Senders `from` email address.
    from: String,
    /// URL to redirect the user to after a successful action.
    #[serde(rename = "resultUrl")]
    result_url: String,
  }
}

mod tenants {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchSettingsResponseDeviceFlow {
    /// Character set used to generate a User Code. Can be `base20` or `digits`.
    charset: String,
    /// Mask used to format a generated User Code into a friendly, readable format.
    mask: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchSettingsResponse {
    /// Guardian page customization.
    guardian_mfa_page: PatchSettingsResponseGuardianMfaPage,
    /// Change Password page customization.
    change_password: PatchSettingsResponseChangePassword,
    /// Selected sandbox version for the extensibility environment.
    sandbox_version: String,
    /// The default absolute redirection uri, must be https and cannot contain a fragment
    default_redirection_uri: String,
    /// End-user support email address.
    support_email: String,
    /// URL of logo to be shown for this tenant (recommended size: 150x150)
    picture_url: String,
    /// Error page customization.
    error_page: PatchSettingsResponseErrorPage,
    /// Name of connection used for password grants at the `/token`endpoint. The following connection types are supported: LDAP, AD, Database Connections, Passwordless, Windows Azure Active Directory, ADFS.
    default_directory: String,
    /// End-user support URL.
    support_url: String,
    /// Flags used to change the behavior of this tenant.
    flags: PatchSettingsResponseFlags,
    /// Friendly name for this tenant.
    friendly_name: String,
    /// Default audience for API authorization.
    default_audience: String,
    /// Device Flow configuration
    device_flow: PatchSettingsResponseDeviceFlow,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchSettingsBodyFlags;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchSettingsResponseGuardianMfaPage {
    ///  Custom Guardian HTML (<a href='https://github.com/Shopify/liquid/wiki/Liquid-for-Designers'>Liquid syntax</a> is supported).
    html: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetSettingsResponseChangePassword {
    /// Custom change password HTML (<a href='https://github.com/Shopify/liquid/wiki/Liquid-for-Designers'>Liquid syntax</a> supported).
    html: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchSettingsResponseErrorPage {
    /// Custom Error HTML (<a href='https://github.com/Shopify/liquid/wiki/Liquid-for-Designers'>Liquid syntax</a> is supported).
    html: String,
    /// URL to redirect to when an error occurs instead of showing the default error page.
    url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetSettingsResponse {
    /// Friendly name for this tenant.
    friendly_name: String,
    /// End-user support email address.
    support_email: String,
    /// Error page customization.
    error_page: GetSettingsResponseErrorPage,
    /// Device Flow configuration
    device_flow: GetSettingsResponseDeviceFlow,
    /// End-user support URL.
    support_url: String,
    /// The default absolute redirection uri, must be https and cannot contain a fragment
    default_redirection_uri: String,
    /// Flags used to change the behavior of this tenant.
    flags: GetSettingsResponseFlags,
    /// Guardian page customization.
    guardian_mfa_page: GetSettingsResponseGuardianMfaPage,
    /// Change Password page customization.
    change_password: GetSettingsResponseChangePassword,
    /// Default audience for API authorization.
    default_audience: String,
    /// Selected sandbox version for the extensibility environment.
    sandbox_version: String,
    /// Name of connection used for password grants at the `/token`endpoint. The following connection types are supported: LDAP, AD, Database Connections, Passwordless, Windows Azure Active Directory, ADFS.
    default_directory: String,
    /// URL of logo to be shown for this tenant (recommended size: 150x150)
    picture_url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchSettingsBodyErrorPage {
    /// URL to redirect to when an error occurs instead of showing the default error page.
    url: String,
    /// Custom Error HTML (<a href='https://github.com/Shopify/liquid/wiki/Liquid-for-Designers'>Liquid syntax</a> is supported).
    html: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchSettingsBody {
    /// Change Password page customization.
    change_password: PatchSettingsBodyChangePassword,
    /// Default audience for API Authorization.
    default_audience: String,
    /// End-user support email.
    support_email: String,
    /// Selected sandbox version for the extensibility environment
    sandbox_version: String,
    /// Number of hours a session will stay valid.
    session_lifetime: i32,
    /// The default absolute redirection uri, must be https and cannot contain a fragment
    default_redirection_uri: String,
    /// Flags used to change the behavior of this tenant.
    flags: PatchSettingsBodyFlags,
    /// Device Flow configuration.
    device_flow: PatchSettingsBodyDeviceFlow,
    /// End-user support url.
    support_url: String,
    /// Guardian page customization.
    guardian_mfa_page: PatchSettingsBodyGuardianMfaPage,
    /// Error page customization.
    error_page: PatchSettingsBodyErrorPage,
    /// Name of connection used for password grants at the `/token` endpoint. The following connection types are supported: LDAP, AD, Database Connections, Passwordless, Windows Azure Active Directory, ADFS.
    default_directory: String,
    /// Number of hours for which a session can be inactive before the user must log in again.
    idle_session_lifetime: i32,
    /// Friendly name for this tenant.
    friendly_name: String,
    /// URL of logo to be shown for this tenant (recommended size: 150x150)
    picture_url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchSettingsResponseChangePassword {
    /// Custom change password HTML (<a href='https://github.com/Shopify/liquid/wiki/Liquid-for-Designers'>Liquid syntax</a> supported).
    html: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetSettingsResponseGuardianMfaPage {
    ///  Custom Guardian HTML (<a href='https://github.com/Shopify/liquid/wiki/Liquid-for-Designers'>Liquid syntax</a> is supported).
    html: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetSettingsResponseFlags;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetSettingsResponseDeviceFlow {
    /// Character set used to generate a User Code. Can be `base20` or `digits`.
    charset: String,
    /// Mask used to format a generated User Code into a friendly, readable format.
    mask: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchSettingsResponseFlags;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchSettingsBodyDeviceFlow {
    /// Character set used to generate a User Code. Can be `base20` or `digits`.
    charset: String,
    /// Mask used to format a generated User Code into a friendly, readable format.
    mask: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchSettingsBodyGuardianMfaPage {
    ///  Custom Guardian HTML (<a href='https://github.com/Shopify/liquid/wiki/Liquid-for-Designers'>Liquid syntax</a> is supported).
    html: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchSettingsBodyChangePassword {
    /// Custom change password HTML (<a href='https://github.com/Shopify/liquid/wiki/Liquid-for-Designers'>Liquid syntax</a> supported).
    html: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetSettingsResponseErrorPage {
    /// Custom Error HTML (<a href='https://github.com/Shopify/liquid/wiki/Liquid-for-Designers'>Liquid syntax</a> is supported).
    html: String,
    /// URL to redirect to when an error occurs instead of showing the default error page.
    url: String,
  }
}

mod prompts {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchPromptsBody {
    /// Which login experience to use. Can be `new` or `classic`.
    universal_login_experience: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetPromptsResponse {
    /// Which login experience to use. Can be `new` or `classic`.
    universal_login_experience: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetCustomTextByLanguageResponse;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PutCustomTextByLanguageBody;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchPromptsResponse {
    /// Which login experience to use. Can be `new` or `classic`.
    universal_login_experience: String,
  }
}

mod jobs {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetJobsByIdResponse {
    /// Status of this job.
    status: String,
    /// ID of this job.
    id: String,
    /// Estimated time remaining before job completes.
    time_left_seconds: i32,
    /// URL to download the result of this job.
    location: String,
    /// connection_id of the connection this job uses.
    connection_id: String,
    /// Completion percentage of this job.
    percentage_done: i32,
    /// Type of job this is.
    #[serde(rename = "type")]
    kind: String,
    /// When this job was created.
    created_at: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostUsersImportsResponse {
    /// When this job was created.
    created_at: String,
    /// Type of job this is.
    #[serde(rename = "type")]
    kind: String,
    /// connection_id of the connection to which users will be imported.
    connection_id: String,
    /// Customer-defined ID.
    external_id: String,
    /// ID of this job.
    id: String,
    /// Status of this job.
    status: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostVerificationEmailResponse {
    /// ID of this job.
    id: String,
    /// Status of this job.
    status: String,
    /// Type of job this is.
    #[serde(rename = "type")]
    kind: String,
    /// When this job was created.
    created_at: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostVerificationEmailBody {
    /// This must be provided to verify primary social, enterprise and passwordless email identities. Also, is needed to verify secondary identities.
    identity: PostVerificationEmailBodyIdentity,
    /// user_id of the user to send the verification email to.
    user_id: String,
    /// client_id of the client (application). If no value provided, the global Client ID will be used.
    client_id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostUsersExportsResponse {
    /// Status of this job.
    status: String,
    /// Format of the file. Must be `json` or `csv`.
    format: String,
    /// Limit the number of records.
    limit: i32,
    /// Type of job this is.
    #[serde(rename = "type")]
    kind: String,
    /// ID of this job.
    id: String,
    /// When this job was created.
    created_at: String,
    /// connection_id of the connection from which users will be exported.
    connection_id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostUsersExportsBody {
    /// connection_id of the connection from which users will be exported.
    connection_id: String,
    /// Limit the number of records.
    limit: i32,
    /// Format of the file. Must be `json` or `csv`.
    format: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostVerificationEmailBodyIdentity {
    /// Identity provider name of the identity (e.g. `google-oauth2`).
    provider: String,
    /// user_id of the identity to be verified.
    user_id: String,
  }
}

mod rules_configs {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PutRulesConfigsByKeyBody {
    /// Value for a rules config variable.
    value: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PutRulesConfigsByKeyResponse {
    /// Key for a rules config variable.
    key: String,
    /// Value for a rules config variable.
    value: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetRulesConfigsResponse {
    /// Key for a rules config variable.
    key: String,
  }
}

mod connections {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostConnectionsBody {
    /// The name of the connection. Must start and end with an alphanumeric character and can only contain alphanumeric characters and '-'. Max length 128
    name: String,
    /// The connection's options (depend on the connection strategy)
    options: PostConnectionsBodyOptions,
    /// Connection name used in the new universal login experience
    display_name: String,
    /// Metadata associated with the connection, in the form of an object with string values (max 255 chars).  Maximum of 10 metadata properties allowed.
    metadata: PostConnectionsBodyMetadata,
    /// The identity provider identifier for the connection
    strategy: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostConnectionsBodyGatewayAuthentication {
    /// The Authorization header type.
    method: String,
    /// The secret to be used for signing tokens.
    secret: String,
    /// The subject to be added to the JWT payload.
    subject: String,
    /// The audience to be added to the JWT payload.
    audience: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetConnectionsResponseOptions;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostConnectionsBodyValidation;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchConnectionsByIdBodyPasswordHistory {
    size: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostConnectionsBodyUsername {
    min: i32,
    max: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetConnectionsByIdResponse {
    /// The name of the connection
    name: String,
    options: GetConnectionsByIdResponseOptions,
    /// Metadata associated with the connection, in the form of an object with string values (max 255 chars).  Maximum of 10 metadata properties allowed.
    metadata: GetConnectionsByIdResponseMetadata,
    /// The connection's identifier
    id: String,
    /// Connection name used in login screen
    display_name: String,
    /// The type of the connection, related to the identity provider
    strategy: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostConnectionsBodyPasswordDictionary;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetConnectionsByIdResponseOptions;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchConnectionsByIdBodyValidation;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchConnectionsByIdBodyPasswordNoPersonalInfo;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchConnectionsByIdBodyGatewayAuthentication {
    /// The secret to be used for signing tokens.
    secret: String,
    /// The Authorization header type.
    method: String,
    /// The subject to be added to the JWT payload.
    subject: String,
    /// The audience to be added to the JWT payload.
    audience: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetConnectionsResponseMetadata;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostConnectionsResponseOptions;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostConnectionsBodyUpstreamParams;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchConnectionsByIdBodyPasswordComplexityOptions {
    /// Minimum password length
    min_length: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchConnectionsByIdResponseOptions;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostConnectionsBodyPasswordNoPersonalInfo;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetConnectionsResponse {
    /// Metadata associated with the connection, in the form of an object with string values (max 255 chars).  Maximum of 10 metadata properties allowed.
    metadata: GetConnectionsResponseMetadata,
    /// The type of the connection, related to the identity provider
    strategy: String,
    /// Connection name used in login screen
    display_name: String,
    options: GetConnectionsResponseOptions,
    /// The name of the connection
    name: String,
    /// The connection's identifier
    id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostConnectionsBodyPasswordComplexityOptions {
    /// Minimum password length
    min_length: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchConnectionsByIdBodyUpstreamParams;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchConnectionsByIdBodyPasswordDictionary;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostConnectionsBodyPasswordHistory {
    size: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchConnectionsByIdBodyMetadata;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostConnectionsResponse {
    /// The name of the connection
    name: String,
    /// Metadata associated with the connection, in the form of an object with string values (max 255 chars).  Maximum of 10 metadata properties allowed.
    metadata: PostConnectionsResponseMetadata,
    /// Connection name used in login screen
    display_name: String,
    /// The connection's identifier
    id: String,
    options: PostConnectionsResponseOptions,
    /// The type of the connection, related to the identity provider
    strategy: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetConnectionsByIdResponseMetadata;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchConnectionsByIdBodyCustomScripts {
    change_password: String,
    create: String,
    get_user: String,
    login: String,
    delete: String,
    verify: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchConnectionsByIdBody {
    /// Metadata associated with the connection, in the form of an object with string values (max 255 chars).  Maximum of 10 metadata properties allowed.
    metadata: PatchConnectionsByIdBodyMetadata,
    /// Connection name used in the new universal login experience
    display_name: String,
    /// The connection's options (depend on the connection strategy)
    options: PatchConnectionsByIdBodyOptions,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostConnectionsResponseMetadata;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostConnectionsBodyCustomScripts {
    delete: String,
    create: String,
    login: String,
    get_user: String,
    verify: String,
    change_password: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchConnectionsByIdBodyOptions {
    /// Password strength level
    #[serde(rename = "passwordPolicy")]
    password_policy: String,
    /// Determines whether the 'name', 'given_name', 'family_name', 'nickname', and 'picture' attributes can be independently updated when using an external IdP. Possible values are 'on_each_login' (default value, it configures the connection to automatically update the root attributes from the external IdP with each user login. When this setting is used, root attributes cannot be independently updated), 'on_first_login' (configures the connection to only set the root attributes on first login, allowing them to be independently updated thereafter)
    set_user_root_attributes: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostConnectionsBodyMetadata;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchConnectionsByIdResponse {
    /// The type of the connection, related to the identity provider
    strategy: String,
    /// Metadata associated with the connection, in the form of an object with string values (max 255 chars).  Maximum of 10 metadata properties allowed.
    metadata: PatchConnectionsByIdResponseMetadata,
    /// Connection name used in login screen
    display_name: String,
    /// The name of the connection
    name: String,
    options: PatchConnectionsByIdResponseOptions,
    /// The connection's identifier
    id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchConnectionsByIdResponseMetadata;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchConnectionsByIdBodyUsername {
    min: i32,
    max: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostConnectionsBodyOptions {
    /// Password strength level
    #[serde(rename = "passwordPolicy")]
    password_policy: String,
    /// Determines whether the 'name', 'given_name', 'family_name', 'nickname', and 'picture' attributes can be independently updated when using an external IdP. Possible values are 'on_each_login' (default value, it configures the connection to automatically update the root attributes from the external IdP with each user login. When this setting is used, root attributes cannot be independently updated), 'on_first_login' (configures the connection to only set the root attributes on first login, allowing them to be independently updated thereafter)
    set_user_root_attributes: String,
  }
}

mod rules {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRulesResponse {
    /// Name of this rule.
    name: String,
    /// ID of this rule.
    id: String,
    /// Code to be executed when this rule runs.
    script: String,
    /// Execution stage of this rule. Can be `login_success`, `login_failure`, or `pre_authorize`.
    stage: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchRulesByIdResponse {
    /// Code to be executed when this rule runs.
    script: String,
    /// Name of this rule.
    name: String,
    /// Execution stage of this rule. Can be `login_success`, `login_failure`, or `pre_authorize`.
    stage: String,
    /// ID of this rule.
    id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRulesBody {
    /// Name of this rule.
    name: String,
    /// Code to be executed when this rule runs.
    script: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetRulesResponse {
    /// Code to be executed when this rule runs.
    script: String,
    /// Execution stage of this rule. Can be `login_success`, `login_failure`, or `pre_authorize`.
    stage: String,
    /// ID of this rule.
    id: String,
    /// Name of this rule.
    name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchRulesByIdBody {
    /// Name of this rule.
    name: String,
    /// Code to be executed when this rule runs.
    script: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetRulesByIdResponse {
    /// Name of this rule.
    name: String,
    /// Code to be executed when this rule runs.
    script: String,
    /// Execution stage of this rule. Can be `login_success`, `login_failure`, or `pre_authorize`.
    stage: String,
    /// ID of this rule.
    id: String,
  }
}

mod blacklists {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetTokensResponse {
    /// jti (unique ID within aud) of the blacklisted JWT.
    jti: String,
    /// JWT's aud claim (the client_id to which the JWT was issued).
    aud: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostTokensBody {
    /// jti (unique ID within aud) of the blacklisted JWT.
    jti: String,
    /// JWT's aud claim (the client_id to which the JWT was issued).
    aud: String,
  }
}

mod log_streams {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostLogStreamsResponse {
    /// The id of the log stream
    id: String,
    /// log stream name
    name: String,
    sink: PostLogStreamsResponseSink,
    /// The status of the log stream. Possible values: `active`, `paused`, `suspended`
    status: String,
    /// log stream type. Possible values: `http`, `eventbridge`
    #[serde(rename = "type")]
    kind: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchLogStreamsByIdResponseSink {
    /// HTTP endpoint
    #[serde(rename = "httpEndpoint")]
    http_endpoint: String,
    /// HTTP Content-Type header
    #[serde(rename = "httpContentType")]
    http_content_type: String,
    /// HTTP JSON format
    #[serde(rename = "httpContentFormat")]
    http_content_format: String,
    /// HTTP Authorization header
    #[serde(rename = "httpAuthorization")]
    http_authorization: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchLogStreamsByIdBodySink {
    /// HTTP endpoint
    #[serde(rename = "httpEndpoint")]
    http_endpoint: String,
    /// HTTP JSON format
    #[serde(rename = "httpContentFormat")]
    http_content_format: String,
    /// HTTP Authorization header
    #[serde(rename = "httpAuthorization")]
    http_authorization: String,
    /// HTTP Content-Type header
    #[serde(rename = "httpContentType")]
    http_content_type: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetLogStreamsResponse;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostLogStreamsResponseSink {
    /// HTTP endpoint
    #[serde(rename = "httpEndpoint")]
    http_endpoint: String,
    /// HTTP Authorization header
    #[serde(rename = "httpAuthorization")]
    http_authorization: String,
    /// HTTP JSON format
    #[serde(rename = "httpContentFormat")]
    http_content_format: String,
    /// HTTP Content-Type header
    #[serde(rename = "httpContentType")]
    http_content_type: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostLogStreamsBodySink {
    /// HTTP endpoint
    #[serde(rename = "httpEndpoint")]
    http_endpoint: String,
    /// HTTP Authorization header
    #[serde(rename = "httpAuthorization")]
    http_authorization: String,
    /// HTTP JSON format
    #[serde(rename = "httpContentFormat")]
    http_content_format: String,
    /// HTTP Content-Type header
    #[serde(rename = "httpContentType")]
    http_content_type: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchLogStreamsByIdResponse {
    /// log stream name
    name: String,
    sink: PatchLogStreamsByIdResponseSink,
    /// The id of the log stream
    id: String,
    /// log stream type. Possible values: `http`, `eventbridge`
    #[serde(rename = "type")]
    kind: String,
    /// The status of the log stream. Possible values: `active`, `paused`, `suspended`
    status: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostLogStreamsBody {
    sink: PostLogStreamsBodySink,
    /// log stream name
    name: String,
    /// log stream type. Possible values: `http`, `eventbridge`
    #[serde(rename = "type")]
    kind: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetLogStreamsByIdResponseSink {
    /// HTTP Content-Type header
    #[serde(rename = "httpContentType")]
    http_content_type: String,
    /// HTTP Authorization header
    #[serde(rename = "httpAuthorization")]
    http_authorization: String,
    /// HTTP endpoint
    #[serde(rename = "httpEndpoint")]
    http_endpoint: String,
    /// HTTP JSON format
    #[serde(rename = "httpContentFormat")]
    http_content_format: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchLogStreamsByIdBody {
    /// log stream name
    name: String,
    /// The status of the log stream. Possible values: `active`, `paused`, `suspended`
    status: String,
    sink: PatchLogStreamsByIdBodySink,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetLogStreamsByIdResponse {
    /// The id of the log stream
    id: String,
    /// log stream name
    name: String,
    /// The status of the log stream. Possible values: `active`, `paused`, `suspended`
    status: String,
    /// log stream type. Possible values: `http`, `eventbridge`
    #[serde(rename = "type")]
    kind: String,
    sink: GetLogStreamsByIdResponseSink,
  }
}

mod logs {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetLogsResponseLocationInfo {
    /// Continent the country is located within. Can be `AF` (Africa), `AN` (Antarctica), `AS` (Asia), `EU` (Europe), `NA` (North America), `OC` (Oceania) or `SA` (South America).
    continent_code: String,
    /// Full country name in English.
    country_name: String,
    /// Two-letter <a href="https://www.iso.org/iso-3166-country-codes.html">Alpha-2 ISO 3166-1</a> country code.
    country_code: String,
    /// Global longitude (vertical) position.
    longitude: String,
    /// Time zone name as found in the <a href="https://www.iana.org/time-zones">tz database</a>.
    time_zone: String,
    /// Three-letter <a href="https://www.iso.org/iso-3166-country-codes.html">Alpha-3 ISO 3166-1</a> country code.
    #[serde(rename = "country_code3")]
    country_code_3: String,
    /// Full city name in English.
    city_name: String,
    /// Global latitude (horizontal) position.
    latitude: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetLogsByIdResponse {
    /// ID of the client (application).
    client_id: String,
    /// User agent string from the client device that caused the event.
    user_agent: String,
    /// Description of this event.
    description: String,
    /// Type of event.
    #[serde(rename = "type")]
    kind: String,
    /// Name of the connection the event relates to.
    connection: String,
    /// Date when the event occurred in ISO 8601 format.
    date: String,
    /// Name of the strategy involved in the event.
    strategy: String,
    /// ID of the connection the event relates to.
    connection_id: String,
    /// IP address of the log event source.
    ip: String,
    /// Unique ID of the event.
    log_id: String,
    /// Hostname the event applies to.
    hostname: String,
    /// Type of strategy involved in the event.
    strategy_type: String,
    /// Information about the location that triggered this event based on the `ip`.
    location_info: GetLogsByIdResponseLocationInfo,
    /// Scope permissions applied to the event.
    scope: String,
    /// ID of the user involved in the event.
    user_id: String,
    /// API audience the event applies to.
    audience: String,
    /// Name of the user involved in the event.
    user_name: String,
    /// Name of the client (application).
    client_name: String,
    /// Additional useful details about this event (structure is dependent upon event type).
    details: GetLogsByIdResponseDetails,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetLogsByIdResponseDetails;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetLogsByIdResponseLocationInfo {
    /// Three-letter <a href="https://www.iso.org/iso-3166-country-codes.html">Alpha-3 ISO 3166-1</a> country code.
    #[serde(rename = "country_code3")]
    country_code_3: String,
    /// Time zone name as found in the <a href="https://www.iana.org/time-zones">tz database</a>.
    time_zone: String,
    /// Global longitude (vertical) position.
    longitude: String,
    /// Two-letter <a href="https://www.iso.org/iso-3166-country-codes.html">Alpha-2 ISO 3166-1</a> country code.
    country_code: String,
    /// Full city name in English.
    city_name: String,
    /// Continent the country is located within. Can be `AF` (Africa), `AN` (Antarctica), `AS` (Asia), `EU` (Europe), `NA` (North America), `OC` (Oceania) or `SA` (South America).
    continent_code: String,
    /// Full country name in English.
    country_name: String,
    /// Global latitude (horizontal) position.
    latitude: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetLogsResponse {
    /// ID of the client (application).
    client_id: String,
    /// Date when the event occurred in ISO 8601 format.
    date: String,
    /// Type of event.
    #[serde(rename = "type")]
    kind: String,
    /// ID of the connection the event relates to.
    connection_id: String,
    /// Scope permissions applied to the event.
    scope: String,
    /// Name of the strategy involved in the event.
    strategy: String,
    /// IP address of the log event source.
    ip: String,
    /// Additional useful details about this event (structure is dependent upon event type).
    details: GetLogsResponseDetails,
    /// Name of the user involved in the event.
    user_name: String,
    /// User agent string from the client device that caused the event.
    user_agent: String,
    /// Name of the client (application).
    client_name: String,
    /// Unique ID of the event.
    log_id: String,
    /// Name of the connection the event relates to.
    connection: String,
    /// ID of the user involved in the event.
    user_id: String,
    /// API audience the event applies to.
    audience: String,
    /// Description of this event.
    description: String,
    /// Hostname the event applies to.
    hostname: String,
    /// Information about the location that triggered this event based on the `ip`.
    location_info: GetLogsResponseLocationInfo,
    /// Type of strategy involved in the event.
    strategy_type: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetLogsResponseDetails;
}

mod emails {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchProviderBodyCredentials;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchProviderBody {
    /// Credentials required to use the provider.
    credentials: PatchProviderBodyCredentials,
    /// Specific provider setting
    settings: PatchProviderBodySettings,
    /// Name of the email provider. Can be `mailgun`, `mandrill`, `sendgrid`, `ses`, `sparkpost`, or `smtp`.
    name: String,
    /// Email address to use as "from" when no other address specified.
    default_from_address: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetProviderResponseCredentials {
    /// SMTP host.
    smtp_host: String,
    /// AWS or SparkPost region.
    region: String,
    /// SMTP port.
    smtp_port: i32,
    /// SMTP username.
    smtp_user: String,
    /// API User.
    api_user: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostProviderResponse {
    /// Specific provider setting
    settings: PostProviderResponseSettings,
    /// Credentials required to use the provider.
    credentials: PostProviderResponseCredentials,
    /// Email address to use as "from" when no other address specified.
    default_from_address: String,
    /// Name of the email provider. Can be `mailgun`, `mandrill`, `sendgrid`, `ses`, `sparkpost`, or `smtp`.
    name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostProviderResponseSettings;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostProviderBodyCredentials;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchProviderResponse {
    /// Email address to use as "from" when no other address specified.
    default_from_address: String,
    /// Name of the email provider. Can be `mailgun`, `mandrill`, `sendgrid`, `ses`, `sparkpost`, or `smtp`.
    name: String,
    /// Specific provider setting
    settings: PatchProviderResponseSettings,
    /// Credentials required to use the provider.
    credentials: PatchProviderResponseCredentials,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetProviderResponse {
    /// Email address to use as "from" when no other address specified.
    default_from_address: String,
    /// Name of the email provider. Can be `mailgun`, `mandrill`, `sendgrid`, `ses`, `sparkpost`, or `smtp`.
    name: String,
    /// Credentials required to use the provider.
    credentials: GetProviderResponseCredentials,
    /// Specific provider setting
    settings: GetProviderResponseSettings,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostProviderBodySettings;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchProviderResponseSettings;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetProviderResponseSettings;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostProviderBody {
    /// Specific provider setting
    settings: PostProviderBodySettings,
    /// Name of the email provider. Can be `mailgun`, `mandrill`, `sendgrid`, `ses`, `sparkpost`, or `smtp`.
    name: String,
    /// Credentials required to use the provider.
    credentials: PostProviderBodyCredentials,
    /// Email address to use as "from" when no other address specified.
    default_from_address: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostProviderResponseCredentials {
    /// API User.
    api_user: String,
    /// SMTP host.
    smtp_host: String,
    /// SMTP port.
    smtp_port: i32,
    /// AWS or SparkPost region.
    region: String,
    /// SMTP username.
    smtp_user: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchProviderResponseCredentials {
    /// API User.
    api_user: String,
    /// SMTP port.
    smtp_port: i32,
    /// SMTP username.
    smtp_user: String,
    /// SMTP host.
    smtp_host: String,
    /// AWS or SparkPost region.
    region: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchProviderBodySettings;
}

mod clients {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseSapApi {
    /// Service account password to use to authenticate API calls to the token endpoint.
    #[serde(rename = "servicePassword")]
    service_password: String,
    /// NameID element of the Subject which can be used to express the user's identity. Defaults to `urn:oasis:names:tc:SAML:1.1:nameid-format:unspecified`.
    #[serde(rename = "nameIdentifierFormat")]
    name_identifier_format: String,
    /// Name of the property in the user object that maps to a SAP username. e.g. `email`.
    #[serde(rename = "usernameAttribute")]
    username_attribute: String,
    /// If activated in the OAuth 2.0 client configuration (transaction SOAUTH2) the SAML attribute client_id must be set and equal the client_id form parameter of the access token request.
    clientid: String,
    /// Requested scope for SAP APIs.
    scope: String,
    /// Your SAP OData server OAuth2 token endpoint URL.
    #[serde(rename = "tokenEndpointUrl")]
    token_endpoint_url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseEgnyte {
    /// Your custom domain found in your Egnyte URL. e.g. `https://acme-org.egnyte.com` would be `acme-org`.
    domain: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseAws {
    /// AWS token lifetime in seconds
    lifetime_in_seconds: i32,
    /// AWS role ARN, e.g. `arn:aws:iam::010616021751:role/foo`
    role: String,
    /// AWS principal ARN, e.g. `arn:aws:iam::010616021751:saml-provider/idpname`
    principal: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyRefreshToken {
    /// Refresh token expiration types, one of: expiring, non-expiring
    expiration_type: String,
    /// Period in seconds where the previous refresh token can be exchanged without triggering breach detection
    leeway: i32,
    /// Refresh token rotation types, one of: rotating, non-rotating
    rotation_type: String,
    /// Period (in seconds) for which refresh tokens will remain valid.
    token_lifetime: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseSalesforceApi {
    /// Consumer Key assigned by Salesforce to the Connected App.
    clientid: String,
    /// Name of the property in the user object that maps to a Salesforce username. e.g. `email`.
    principal: String,
    /// Community name.
    #[serde(rename = "communityName")]
    community_name: String,
    /// Community url section.
    community_url_section: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseApple;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseScopes;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseNewrelic {
    /// Your New Relic Account ID found in your New Relic URL after the `/accounts/` path. e.g. `https://rpm.newrelic.com/accounts/123456/query` would be `123456`.
    account: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseMobile;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyJwtConfiguration {
    /// Algorithm used to sign JWTs. Can be `HS256` or `RS256`.
    alg: String,
    /// Number of seconds the JWT will be valid for (affects `exp` claim).
    lifetime_in_seconds: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseApple;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyLayer {
    /// Provider ID of your Layer account
    #[serde(rename = "providerId")]
    provider_id: String,
    /// Optional expiration in minutes for the generated token. Defaults to 5 minutes.
    expiration: i32,
    /// Private key for signing the Layer token.
    #[serde(rename = "privateKey")]
    private_key: String,
    /// Authentication Key identifier used to sign the Layer token.
    #[serde(rename = "keyId")]
    key_id: String,
    /// Name of the property used as the unique user id in Layer. If not specified `user_id` is used.
    principal: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseRms {
    /// URL of your Rights Management Server. It can be internal or external, but users will have to be able to reach it.
    url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodySsoIntegration {
    /// SSO integration version installed
    version: String,
    /// SSO integration name
    name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyAddons;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyAzureBlob {
    /// Access key associated with this storage account.
    #[serde(rename = "storageAccessKey")]
    storage_access_key: String,
    /// Expiration in minutes for the generated token (default of 5 minutes).
    expiration: i32,
    /// Shared access policy identifier defined in your storage account resource.
    #[serde(rename = "signedIdentifier")]
    signed_identifier: String,
    /// Your Azure storage account name. Usually first segment in your Azure storage URL. e.g. `https://acme-org.blob.core.windows.net` would be the account name `acme-org`.
    #[serde(rename = "accountName")]
    account_name: String,
    /// Container to request a token for. e.g. `my-container`.
    #[serde(rename = "containerName")]
    container_name: String,
    /// Entity to request a token for. e.g. `my-blob`. If blank the computed SAS will apply to the entire storage container.
    #[serde(rename = "blobName")]
    blob_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyApple;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodySentry {
    /// Generated slug for your Sentry organization. Found in your Sentry URL. e.g. `https://sentry.acme.com/acme-org/` would be `acme-org`.
    org_slug: String,
    /// URL prefix only if running Sentry Community Edition, otherwise leave should be blank.
    base_url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseAzureBlob {
    /// Container to request a token for. e.g. `my-container`.
    #[serde(rename = "containerName")]
    container_name: String,
    /// Your Azure storage account name. Usually first segment in your Azure storage URL. e.g. `https://acme-org.blob.core.windows.net` would be the account name `acme-org`.
    #[serde(rename = "accountName")]
    account_name: String,
    /// Entity to request a token for. e.g. `my-blob`. If blank the computed SAS will apply to the entire storage container.
    #[serde(rename = "blobName")]
    blob_name: String,
    /// Access key associated with this storage account.
    #[serde(rename = "storageAccessKey")]
    storage_access_key: String,
    /// Shared access policy identifier defined in your storage account resource.
    #[serde(rename = "signedIdentifier")]
    signed_identifier: String,
    /// Expiration in minutes for the generated token (default of 5 minutes).
    expiration: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseCloudbees;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseAws {
    /// AWS role ARN, e.g. `arn:aws:iam::010616021751:role/foo`
    role: String,
    /// AWS principal ARN, e.g. `arn:aws:iam::010616021751:saml-provider/idpname`
    principal: String,
    /// AWS token lifetime in seconds
    lifetime_in_seconds: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseApple;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyApple;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseSharepoint {
    /// Internal SharePoint application URL.
    url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyRms {
    /// URL of your Rights Management Server. It can be internal or external, but users will have to be able to reach it.
    url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyWams {
    /// Your master key for Windows Azure Mobile Services.
    masterkey: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseBox;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyZendesk {
    /// Zendesk account name usually first segment in your Zendesk URL. e.g. `https://acme-org.zendesk.com` would be `acme-org`.
    #[serde(rename = "accountName")]
    account_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseMscrm {
    /// Microsoft Dynamics CRM application URL.
    url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseZoom {
    /// Zoom account name usually first segment of your Zoom URL, e.g. `https://acme-org.zoom.us` would be `acme-org`.
    account: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyZendesk {
    /// Zendesk account name usually first segment in your Zendesk URL. e.g. `https://acme-org.zendesk.com` would be `acme-org`.
    #[serde(rename = "accountName")]
    account_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseFirebase {
    /// ID of the Service Account you have created (shown as `client_email` in the generated JSON file, SDK v3+ tokens only).
    client_email: String,
    /// Optional ID of the private key to obtain kid header in the issued token (SDK v3+ tokens only).
    private_key_id: String,
    /// Private Key for signing the token (SDK v3+ tokens only).
    private_key: String,
    /// Optional expiration in seconds for the generated token. Defaults to 3600 seconds (SDK v3+ tokens only).
    lifetime_in_seconds: i32,
    /// Google Firebase Secret. (SDK 2 only).
    secret: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseJwtConfiguration {
    /// Algorithm used to sign JWTs. Can be `HS256` or `RS256`.
    alg: String,
    /// Number of seconds the JWT will be valid for (affects `exp` claim).
    lifetime_in_seconds: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyClientMetadata;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseSalesforce {
    /// Arbitrary logical URL that identifies the Saleforce resource. e.g. `https://acme-org.com`.
    entity_id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseScopes;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseMappings;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseSalesforceSandboxApi {
    /// Community name.
    #[serde(rename = "communityName")]
    community_name: String,
    /// Consumer Key assigned by Salesforce to the Connected App.
    clientid: String,
    /// Name of the property in the user object that maps to a Salesforce username. e.g. `email`.
    principal: String,
    /// Community url section.
    community_url_section: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseCloudbees;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseAzureSb {
    /// Entity you want to request a token for. e.g. `my-queue`.'
    #[serde(rename = "entityPath")]
    entity_path: String,
    /// Optional expiration in minutes for the generated token. Defaults to 5 minutes.
    expiration: i32,
    /// Primary Key associated with your shared access policy.
    #[serde(rename = "sasKey")]
    sas_key: String,
    /// Your Azure Service Bus namespace. Usually the first segment of your Service Bus URL (e.g. `https://acme-org.servicebus.windows.net` would be `acme-org`).
    namespace: String,
    /// Your shared access policy name defined in your Service Bus entity.
    #[serde(rename = "sasKeyName")]
    sas_key_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseOffice365 {
    /// Optional Auth0 database connection for testing an already-configured Office 365 tenant.
    connection: String,
    /// Your Office 365 domain name. e.g. `acme-org.com`.
    domain: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseAndroid {
    /// App package name found in AndroidManifest.xml.
    app_package_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseEchosign {
    /// Your custom domain found in your EchoSign URL. e.g. `https://acme-org.echosign.com` would be `acme-org`.
    domain: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseSalesforceApi {
    /// Community url section.
    community_url_section: String,
    /// Name of the property in the user object that maps to a Salesforce username. e.g. `email`.
    principal: String,
    /// Community name.
    #[serde(rename = "communityName")]
    community_name: String,
    /// Consumer Key assigned by Salesforce to the Connected App.
    clientid: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseBox;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyMappings;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyIos {
    /// Assigned by the developer to the app as its unique identifier inside the store, usually is a reverse domain plus the app name: <code>com.you.MyApp</code>
    app_bundle_identifier: String,
    /// Identifier assigned to the account that signs and upload the app to the store
    team_id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseJwtConfiguration {
    /// Number of seconds the JWT will be valid for (affects `exp` claim).
    lifetime_in_seconds: i32,
    /// Algorithm used to sign JWTs. Can be `HS256` or `RS256`.
    alg: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseRms {
    /// URL of your Rights Management Server. It can be internal or external, but users will have to be able to reach it.
    url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseLayer {
    /// Provider ID of your Layer account
    #[serde(rename = "providerId")]
    provider_id: String,
    /// Authentication Key identifier used to sign the Layer token.
    #[serde(rename = "keyId")]
    key_id: String,
    /// Private key for signing the Layer token.
    #[serde(rename = "privateKey")]
    private_key: String,
    /// Name of the property used as the unique user id in Layer. If not specified `user_id` is used.
    principal: String,
    /// Optional expiration in minutes for the generated token. Defaults to 5 minutes.
    expiration: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseMobile;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseEchosign {
    /// Your custom domain found in your EchoSign URL. e.g. `https://acme-org.echosign.com` would be `acme-org`.
    domain: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseNewrelic {
    /// Your New Relic Account ID found in your New Relic URL after the `/accounts/` path. e.g. `https://rpm.newrelic.com/accounts/123456/query` would be `123456`.
    account: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyOffice365 {
    /// Optional Auth0 database connection for testing an already-configured Office 365 tenant.
    connection: String,
    /// Your Office 365 domain name. e.g. `acme-org.com`.
    domain: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseWsfed;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseSalesforce {
    /// Arbitrary logical URL that identifies the Saleforce resource. e.g. `https://acme-org.com`.
    entity_id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyAws {
    /// AWS role ARN, e.g. `arn:aws:iam::010616021751:role/foo`
    role: String,
    /// AWS principal ARN, e.g. `arn:aws:iam::010616021751:saml-provider/idpname`
    principal: String,
    /// AWS token lifetime in seconds
    lifetime_in_seconds: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseRefreshToken {
    /// Refresh token expiration types, one of: expiring, non-expiring
    expiration_type: String,
    /// Period in seconds where the previous refresh token can be exchanged without triggering breach detection
    leeway: i32,
    /// Refresh token rotation types, one of: rotating, non-rotating
    rotation_type: String,
    /// Period (in seconds) for which refresh tokens will remain valid.
    token_lifetime: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodySapApi {
    /// If activated in the OAuth 2.0 client configuration (transaction SOAUTH2) the SAML attribute client_id must be set and equal the client_id form parameter of the access token request.
    clientid: String,
    /// NameID element of the Subject which can be used to express the user's identity. Defaults to `urn:oasis:names:tc:SAML:1.1:nameid-format:unspecified`.
    #[serde(rename = "nameIdentifierFormat")]
    name_identifier_format: String,
    /// Service account password to use to authenticate API calls to the token endpoint.
    #[serde(rename = "servicePassword")]
    service_password: String,
    /// Requested scope for SAP APIs.
    scope: String,
    /// Your SAP OData server OAuth2 token endpoint URL.
    #[serde(rename = "tokenEndpointUrl")]
    token_endpoint_url: String,
    /// Name of the property in the user object that maps to a SAP username. e.g. `email`.
    #[serde(rename = "usernameAttribute")]
    username_attribute: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyRefreshToken {
    /// Period (in seconds) for which refresh tokens will remain valid.
    token_lifetime: i32,
    /// Refresh token rotation types, one of: rotating, non-rotating
    rotation_type: String,
    /// Period in seconds where the previous refresh token can be exchanged without triggering breach detection
    leeway: i32,
    /// Refresh token expiration types, one of: expiring, non-expiring
    expiration_type: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseWsfed;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseSpringcm {
    /// SpringCM ACS URL, e.g. `https://na11.springcm.com/atlas/sso/SSOEndpoint.ashx`.
    acsurl: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseClientMetadata;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyEgnyte {
    /// Your custom domain found in your Egnyte URL. e.g. `https://acme-org.egnyte.com` would be `acme-org`.
    domain: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseAzureSb {
    /// Your Azure Service Bus namespace. Usually the first segment of your Service Bus URL (e.g. `https://acme-org.servicebus.windows.net` would be `acme-org`).
    namespace: String,
    /// Entity you want to request a token for. e.g. `my-queue`.'
    #[serde(rename = "entityPath")]
    entity_path: String,
    /// Your shared access policy name defined in your Service Bus entity.
    #[serde(rename = "sasKeyName")]
    sas_key_name: String,
    /// Optional expiration in minutes for the generated token. Defaults to 5 minutes.
    expiration: i32,
    /// Primary Key associated with your shared access policy.
    #[serde(rename = "sasKey")]
    sas_key: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseMobile;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseIos {
    /// Identifier assigned to the Apple account that signs and uploads the app to the store.
    team_id: String,
    /// Assigned by developer to the app as its unique identifier inside the store. Usually this is a reverse domain plus the app name, e.g. `com.you.MyApp`.
    app_bundle_identifier: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseSentry {
    /// Generated slug for your Sentry organization. Found in your Sentry URL. e.g. `https://sentry.acme.com/acme-org/` would be `acme-org`.
    org_slug: String,
    /// URL prefix only if running Sentry Community Edition, otherwise leave should be blank.
    base_url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseFirebase {
    /// Optional ID of the private key to obtain kid header in the issued token (SDK v3+ tokens only).
    private_key_id: String,
    /// Optional expiration in seconds for the generated token. Defaults to 3600 seconds (SDK v3+ tokens only).
    lifetime_in_seconds: i32,
    /// Google Firebase Secret. (SDK 2 only).
    secret: String,
    /// Private Key for signing the token (SDK v3+ tokens only).
    private_key: String,
    /// ID of the Service Account you have created (shown as `client_email` in the generated JSON file, SDK v3+ tokens only).
    client_email: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyFacebook;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseAws {
    /// AWS principal ARN, e.g. `arn:aws:iam::010616021751:saml-provider/idpname`
    principal: String,
    /// AWS role ARN, e.g. `arn:aws:iam::010616021751:role/foo`
    role: String,
    /// AWS token lifetime in seconds
    lifetime_in_seconds: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseSentry {
    /// Generated slug for your Sentry organization. Found in your Sentry URL. e.g. `https://sentry.acme.com/acme-org/` would be `acme-org`.
    org_slug: String,
    /// URL prefix only if running Sentry Community Edition, otherwise leave should be blank.
    base_url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseEncryptionKey {
    /// Encryption Public RSA Key.
    #[serde(rename = "pub")]
    public: String,
    /// Encryption certificate for public key in X.590 (.CER) format.
    cert: String,
    /// Encryption certificate name for this certificate in the format `/CN={domain}`.
    subject: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseZendesk {
    /// Zendesk account name usually first segment in your Zendesk URL. e.g. `https://acme-org.zendesk.com` would be `acme-org`.
    #[serde(rename = "accountName")]
    account_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyMscrm {
    /// Microsoft Dynamics CRM application URL.
    url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseClientMetadata;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyMappings;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseNewrelic {
    /// Your New Relic Account ID found in your New Relic URL after the `/accounts/` path. e.g. `https://rpm.newrelic.com/accounts/123456/query` would be `123456`.
    account: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseMappings;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseNativeSocialLogin;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyFirebase {
    /// ID of the Service Account you have created (shown as `client_email` in the generated JSON file, SDK v3+ tokens only).
    client_email: String,
    /// Optional expiration in seconds for the generated token. Defaults to 3600 seconds (SDK v3+ tokens only).
    lifetime_in_seconds: i32,
    /// Private Key for signing the token (SDK v3+ tokens only).
    private_key: String,
    /// Optional ID of the private key to obtain kid header in the issued token (SDK v3+ tokens only).
    private_key_id: String,
    /// Google Firebase Secret. (SDK 2 only).
    secret: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponse {
    /// Name of this client (min length: 1 character, does not allow `<` or `>`).
    name: String,
    /// URL of the logo to display for this client. Recommended size is 150x150 pixels.
    logo_uri: String,
    /// Addons enabled for this client and their associated configurations.
    addons: GetClientsResponseAddons,
    /// Defines the requested authentication method for the token endpoint. Can be `none` (public client without a client secret), `client_secret_post` (client uses HTTP POST parameters), or `client_secret_basic` (client uses HTTP Basic).
    token_endpoint_auth_method: String,
    /// Initiate login uri, must be https and cannot contain a fragment
    initiate_login_uri: String,
    /// HTML form template to be used for WS-Federation.
    form_template: String,
    /// Free text description of this client (max length: 140 characters).
    description: String,
    /// Configuration related to JWTs for the client.
    jwt_configuration: GetClientsResponseJwtConfiguration,
    /// The content (HTML, CSS, JS) of the custom login page. (Used on Previews)
    custom_login_page_preview: String,
    /// Configure native social settings
    native_social_login: GetClientsResponseNativeSocialLogin,
    /// Client secret (which you must not make public).
    client_secret: String,
    /// ID of this client.
    client_id: String,
    /// The content (HTML, CSS, JS) of the custom login page.
    custom_login_page: String,
    /// Additional configuration for native mobile apps.
    mobile: GetClientsResponseMobile,
    /// Refresh token configuration
    refresh_token: GetClientsResponseRefreshToken,
    /// Encryption used for WsFed responses with this client.
    encryption_key: GetClientsResponseEncryptionKey,
    /// Metadata associated with the client, in the form of an object with string values (max 255 chars).  Maximum of 10 metadata properties allowed.
    client_metadata: GetClientsResponseClientMetadata,
    /// Name of the tenant this client belongs to.
    tenant: String,
    /// URL of the location in your site where the cross origin verification takes place for the cross-origin auth flow when performing Auth in your own domain instead of Auth0 hosted login page.
    cross_origin_loc: String,
    /// Type of client used to determine which settings are applicable. Can be `spa`, `native`, `non_interactive`, or `regular_web`.
    app_type: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseScopes;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseSalesforceApi {
    /// Community url section.
    community_url_section: String,
    /// Name of the property in the user object that maps to a Salesforce username. e.g. `email`.
    principal: String,
    /// Community name.
    #[serde(rename = "communityName")]
    community_name: String,
    /// Consumer Key assigned by Salesforce to the Connected App.
    clientid: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseSsoIntegration {
    /// SSO integration name
    name: String,
    /// SSO integration version installed
    version: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodySalesforce {
    /// Arbitrary logical URL that identifies the Saleforce resource. e.g. `https://acme-org.com`.
    entity_id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseZendesk {
    /// Zendesk account name usually first segment in your Zendesk URL. e.g. `https://acme-org.zendesk.com` would be `acme-org`.
    #[serde(rename = "accountName")]
    account_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodySharepoint {
    /// Internal SharePoint application URL.
    url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyClientMetadata;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseIos {
    /// Identifier assigned to the Apple account that signs and uploads the app to the store.
    team_id: String,
    /// Assigned by developer to the app as its unique identifier inside the store. Usually this is a reverse domain plus the app name, e.g. `com.you.MyApp`.
    app_bundle_identifier: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseJwtConfiguration {
    /// Number of seconds the JWT will be valid for (affects `exp` claim).
    lifetime_in_seconds: i32,
    /// Algorithm used to sign JWTs. Can be `HS256` or `RS256`.
    alg: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseMobile;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseSapApi {
    /// If activated in the OAuth 2.0 client configuration (transaction SOAUTH2) the SAML attribute client_id must be set and equal the client_id form parameter of the access token request.
    clientid: String,
    /// Name of the property in the user object that maps to a SAP username. e.g. `email`.
    #[serde(rename = "usernameAttribute")]
    username_attribute: String,
    /// Service account password to use to authenticate API calls to the token endpoint.
    #[serde(rename = "servicePassword")]
    service_password: String,
    /// Requested scope for SAP APIs.
    scope: String,
    /// NameID element of the Subject which can be used to express the user's identity. Defaults to `urn:oasis:names:tc:SAML:1.1:nameid-format:unspecified`.
    #[serde(rename = "nameIdentifierFormat")]
    name_identifier_format: String,
    /// Your SAP OData server OAuth2 token endpoint URL.
    #[serde(rename = "tokenEndpointUrl")]
    token_endpoint_url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseWams {
    /// Your master key for Windows Azure Mobile Services.
    masterkey: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyBox;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseEgnyte {
    /// Your custom domain found in your Egnyte URL. e.g. `https://acme-org.egnyte.com` would be `acme-org`.
    domain: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseAzureBlob {
    /// Access key associated with this storage account.
    #[serde(rename = "storageAccessKey")]
    storage_access_key: String,
    /// Expiration in minutes for the generated token (default of 5 minutes).
    expiration: i32,
    /// Container to request a token for. e.g. `my-container`.
    #[serde(rename = "containerName")]
    container_name: String,
    /// Entity to request a token for. e.g. `my-blob`. If blank the computed SAS will apply to the entire storage container.
    #[serde(rename = "blobName")]
    blob_name: String,
    /// Shared access policy identifier defined in your storage account resource.
    #[serde(rename = "signedIdentifier")]
    signed_identifier: String,
    /// Your Azure storage account name. Usually first segment in your Azure storage URL. e.g. `https://acme-org.blob.core.windows.net` would be the account name `acme-org`.
    #[serde(rename = "accountName")]
    account_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyWsfed;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponse {
    /// ID of this client.
    client_id: String,
    /// Encryption used for WsFed responses with this client.
    encryption_key: PatchClientsByIdResponseEncryptionKey,
    /// The content (HTML, CSS, JS) of the custom login page. (Used on Previews)
    custom_login_page_preview: String,
    /// Additional configuration for native mobile apps.
    mobile: PatchClientsByIdResponseMobile,
    /// Defines the requested authentication method for the token endpoint. Can be `none` (public client without a client secret), `client_secret_post` (client uses HTTP POST parameters), or `client_secret_basic` (client uses HTTP Basic).
    token_endpoint_auth_method: String,
    /// URL of the logo to display for this client. Recommended size is 150x150 pixels.
    logo_uri: String,
    /// Refresh token configuration
    refresh_token: PatchClientsByIdResponseRefreshToken,
    /// HTML form template to be used for WS-Federation.
    form_template: String,
    /// Client secret (which you must not make public).
    client_secret: String,
    /// Free text description of this client (max length: 140 characters).
    description: String,
    /// Configure native social settings
    native_social_login: PatchClientsByIdResponseNativeSocialLogin,
    /// Initiate login uri, must be https and cannot contain a fragment
    initiate_login_uri: String,
    /// Type of client used to determine which settings are applicable. Can be `spa`, `native`, `non_interactive`, or `regular_web`.
    app_type: String,
    /// Addons enabled for this client and their associated configurations.
    addons: PatchClientsByIdResponseAddons,
    /// The content (HTML, CSS, JS) of the custom login page.
    custom_login_page: String,
    /// Name of the tenant this client belongs to.
    tenant: String,
    /// Configuration related to JWTs for the client.
    jwt_configuration: PatchClientsByIdResponseJwtConfiguration,
    /// Metadata associated with the client, in the form of an object with string values (max 255 chars).  Maximum of 10 metadata properties allowed.
    client_metadata: PatchClientsByIdResponseClientMetadata,
    /// Name of this client (min length: 1 character, does not allow `<` or `>`).
    name: String,
    /// URL of the location in your site where the cross origin verification takes place for the cross-origin auth flow when performing Auth in your own domain instead of Auth0 hosted login page.
    cross_origin_loc: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseSalesforceSandboxApi {
    /// Community name.
    #[serde(rename = "communityName")]
    community_name: String,
    /// Consumer Key assigned by Salesforce to the Connected App.
    clientid: String,
    /// Name of the property in the user object that maps to a Salesforce username. e.g. `email`.
    principal: String,
    /// Community url section.
    community_url_section: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseApple;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseZendesk {
    /// Zendesk account name usually first segment in your Zendesk URL. e.g. `https://acme-org.zendesk.com` would be `acme-org`.
    #[serde(rename = "accountName")]
    account_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyCloudbees;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseRefreshToken {
    /// Refresh token expiration types, one of: expiring, non-expiring
    expiration_type: String,
    /// Period in seconds where the previous refresh token can be exchanged without triggering breach detection
    leeway: i32,
    /// Period (in seconds) for which refresh tokens will remain valid.
    token_lifetime: i32,
    /// Refresh token rotation types, one of: rotating, non-rotating
    rotation_type: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseOffice365 {
    /// Your Office 365 domain name. e.g. `acme-org.com`.
    domain: String,
    /// Optional Auth0 database connection for testing an already-configured Office 365 tenant.
    connection: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseSharepoint {
    /// Internal SharePoint application URL.
    url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyFacebook;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseFirebase {
    /// Private Key for signing the token (SDK v3+ tokens only).
    private_key: String,
    /// Optional expiration in seconds for the generated token. Defaults to 3600 seconds (SDK v3+ tokens only).
    lifetime_in_seconds: i32,
    /// ID of the Service Account you have created (shown as `client_email` in the generated JSON file, SDK v3+ tokens only).
    client_email: String,
    /// Google Firebase Secret. (SDK 2 only).
    secret: String,
    /// Optional ID of the private key to obtain kid header in the issued token (SDK v3+ tokens only).
    private_key_id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyAddons;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseAzureBlob {
    /// Expiration in minutes for the generated token (default of 5 minutes).
    expiration: i32,
    /// Access key associated with this storage account.
    #[serde(rename = "storageAccessKey")]
    storage_access_key: String,
    /// Shared access policy identifier defined in your storage account resource.
    #[serde(rename = "signedIdentifier")]
    signed_identifier: String,
    /// Entity to request a token for. e.g. `my-blob`. If blank the computed SAS will apply to the entire storage container.
    #[serde(rename = "blobName")]
    blob_name: String,
    /// Container to request a token for. e.g. `my-container`.
    #[serde(rename = "containerName")]
    container_name: String,
    /// Your Azure storage account name. Usually first segment in your Azure storage URL. e.g. `https://acme-org.blob.core.windows.net` would be the account name `acme-org`.
    #[serde(rename = "accountName")]
    account_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponse {
    /// The content (HTML, CSS, JS) of the custom login page. (Used on Previews)
    custom_login_page_preview: String,
    /// URL of the logo to display for this client. Recommended size is 150x150 pixels.
    logo_uri: String,
    /// Addons enabled for this client and their associated configurations.
    addons: PostClientsResponseAddons,
    /// Free text description of this client (max length: 140 characters).
    description: String,
    /// The content (HTML, CSS, JS) of the custom login page.
    custom_login_page: String,
    /// Type of client used to determine which settings are applicable. Can be `spa`, `native`, `non_interactive`, or `regular_web`.
    app_type: String,
    /// Refresh token configuration
    refresh_token: PostClientsResponseRefreshToken,
    /// Name of the tenant this client belongs to.
    tenant: String,
    /// Metadata associated with the client, in the form of an object with string values (max 255 chars).  Maximum of 10 metadata properties allowed.
    client_metadata: PostClientsResponseClientMetadata,
    /// Configuration related to JWTs for the client.
    jwt_configuration: PostClientsResponseJwtConfiguration,
    /// Configure native social settings
    native_social_login: PostClientsResponseNativeSocialLogin,
    /// ID of this client.
    client_id: String,
    /// URL of the location in your site where the cross origin verification takes place for the cross-origin auth flow when performing Auth in your own domain instead of Auth0 hosted login page.
    cross_origin_loc: String,
    /// Name of this client (min length: 1 character, does not allow `<` or `>`).
    name: String,
    /// Defines the requested authentication method for the token endpoint. Can be `none` (public client without a client secret), `client_secret_post` (client uses HTTP POST parameters), or `client_secret_basic` (client uses HTTP Basic).
    token_endpoint_auth_method: String,
    /// Initiate login uri, must be https and cannot contain a fragment
    initiate_login_uri: String,
    /// Encryption used for WsFed responses with this client.
    encryption_key: PostClientsResponseEncryptionKey,
    /// HTML form template to be used for WS-Federation.
    form_template: String,
    /// Client secret (which you must not make public).
    client_secret: String,
    /// Additional configuration for native mobile apps.
    mobile: PostClientsResponseMobile,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseSsoIntegration {
    /// SSO integration version installed
    version: String,
    /// SSO integration name
    name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseWsfed;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyJwtConfiguration {
    /// The amount of time (in seconds) that the token will be valid after being issued
    lifetime_in_seconds: i32,
    /// The algorithm used to sign the JsonWebToken
    alg: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseSlack {
    /// Slack team name.
    team: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseMappings;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseSapApi {
    /// If activated in the OAuth 2.0 client configuration (transaction SOAUTH2) the SAML attribute client_id must be set and equal the client_id form parameter of the access token request.
    clientid: String,
    /// NameID element of the Subject which can be used to express the user's identity. Defaults to `urn:oasis:names:tc:SAML:1.1:nameid-format:unspecified`.
    #[serde(rename = "nameIdentifierFormat")]
    name_identifier_format: String,
    /// Your SAP OData server OAuth2 token endpoint URL.
    #[serde(rename = "tokenEndpointUrl")]
    token_endpoint_url: String,
    /// Name of the property in the user object that maps to a SAP username. e.g. `email`.
    #[serde(rename = "usernameAttribute")]
    username_attribute: String,
    /// Requested scope for SAP APIs.
    scope: String,
    /// Service account password to use to authenticate API calls to the token endpoint.
    #[serde(rename = "servicePassword")]
    service_password: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseDropbox;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBody {
    /// Configuration related to JWTs for the client.
    jwt_configuration: PostClientsBodyJwtConfiguration,
    /// HTML form template to be used for WS-Federation.
    form_template: String,
    /// Addons enabled for this client and their associated configurations.
    addons: PostClientsBodyAddons,
    /// URL of the logo to display for this client. Recommended size is 150x150 pixels.
    logo_uri: String,
    /// The content (HTML, CSS, JS) of the custom login page. (Used on Previews)
    custom_login_page_preview: String,
    /// Metadata associated with the client, in the form of an object with string values (max 255 chars).  Maximum of 10 metadata properties allowed.
    client_metadata: PostClientsBodyClientMetadata,
    /// Refresh token configuration
    refresh_token: PostClientsBodyRefreshToken,
    /// Name of this client (min length: 1 character, does not allow `<` or `>`).
    name: String,
    /// Initiate login uri, must be https and cannot contain a fragment
    initiate_login_uri: String,
    /// Encryption used for WsFed responses with this client.
    encryption_key: PostClientsBodyEncryptionKey,
    /// Additional configuration for native mobile apps.
    mobile: PostClientsBodyMobile,
    /// Type of client used to determine which settings are applicable. Can be `spa`, `native`, `non_interactive`, or `regular_web`.
    app_type: String,
    /// Free text description of this client (max length: 140 characters).
    description: String,
    /// URL of the location in your site where the cross origin verification takes place for the cross-origin auth flow when performing Auth in your own domain instead of Auth0 hosted login page.
    cross_origin_loc: String,
    /// Defines the requested authentication method for the token endpoint. Can be `none` (public client without a client secret), `client_secret_post` (client uses HTTP POST parameters), or `client_secret_basic` (client uses HTTP Basic).
    token_endpoint_auth_method: String,
    /// Configure native social settings
    native_social_login: PostClientsBodyNativeSocialLogin,
    /// The content (HTML, CSS, JS) of the custom login page.
    custom_login_page: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBody {
    /// The type of application this client represents
    app_type: String,
    /// Free text description of the purpose of the Client. (Max character length: <code>140</code>)
    description: String,
    /// The client's encryption key
    encryption_key: PatchClientsByIdBodyEncryptionKey,
    /// The URL of the client logo (recommended size: 150x150)
    logo_uri: String,
    custom_login_page_preview: String,
    /// The content (HTML, CSS, JS) of the custom login page
    custom_login_page: String,
    /// Metadata associated with the client, in the form of an object with string values (max 255 chars).  Maximum of 10 metadata properties allowed.
    client_metadata: PatchClientsByIdBodyClientMetadata,
    /// Configuration related to native mobile apps
    mobile: PatchClientsByIdBodyMobile,
    /// Initiate login uri, must be https and cannot contain a fragment
    initiate_login_uri: String,
    /// Refresh token configuration
    refresh_token: PatchClientsByIdBodyRefreshToken,
    /// Defines the requested authentication method for the token endpoint. Possible values are 'none' (public client without a client secret), 'client_secret_post' (client uses HTTP POST parameters) or 'client_secret_basic' (client uses HTTP Basic)
    token_endpoint_auth_method: String,
    /// URL for the location in your site where the cross origin verification takes place for the cross-origin auth flow when performing Auth in your own domain instead of Auth0 hosted login page.
    cross_origin_loc: String,
    /// The name of the client. Must contain at least one character. Does not allow '<' or '>'.
    name: String,
    /// Form template for WS-Federation protocol
    form_template: String,
    /// Configure native social settings
    native_social_login: PatchClientsByIdBodyNativeSocialLogin,
    /// Addons enabled for this client and their associated configurations.
    addons: PatchClientsByIdBodyAddons,
    /// An object that holds settings related to how JWTs are created
    jwt_configuration: PatchClientsByIdBodyJwtConfiguration,
    /// The secret used to sign tokens for the client
    client_secret: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseSharepoint {
    /// Internal SharePoint application URL.
    url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyScopes;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyAzureBlob {
    /// Access key associated with this storage account.
    #[serde(rename = "storageAccessKey")]
    storage_access_key: String,
    /// Entity to request a token for. e.g. `my-blob`. If blank the computed SAS will apply to the entire storage container.
    #[serde(rename = "blobName")]
    blob_name: String,
    /// Your Azure storage account name. Usually first segment in your Azure storage URL. e.g. `https://acme-org.blob.core.windows.net` would be the account name `acme-org`.
    #[serde(rename = "accountName")]
    account_name: String,
    /// Expiration in minutes for the generated token (default of 5 minutes).
    expiration: i32,
    /// Container to request a token for. e.g. `my-container`.
    #[serde(rename = "containerName")]
    container_name: String,
    /// Shared access policy identifier defined in your storage account resource.
    #[serde(rename = "signedIdentifier")]
    signed_identifier: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseOffice365 {
    /// Your Office 365 domain name. e.g. `acme-org.com`.
    domain: String,
    /// Optional Auth0 database connection for testing an already-configured Office 365 tenant.
    connection: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseSalesforceSandboxApi {
    /// Community url section.
    community_url_section: String,
    /// Name of the property in the user object that maps to a Salesforce username. e.g. `email`.
    principal: String,
    /// Consumer Key assigned by Salesforce to the Connected App.
    clientid: String,
    /// Community name.
    #[serde(rename = "communityName")]
    community_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyConcur;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodySpringcm {
    /// SpringCM ACS URL, e.g. `https://na11.springcm.com/atlas/sso/SSOEndpoint.ashx`.
    acsurl: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodySsoIntegration {
    /// SSO integration version installed
    version: String,
    /// SSO integration name
    name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseSalesforce {
    /// Arbitrary logical URL that identifies the Saleforce resource. e.g. `https://acme-org.com`.
    entity_id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseConcur;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyNativeSocialLogin;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseNativeSocialLogin;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseAndroid {
    /// App package name found in AndroidManifest.xml.
    app_package_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyEchosign {
    /// Your custom domain found in your EchoSign URL. e.g. `https://acme-org.echosign.com` would be `acme-org`.
    domain: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseConcur;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyCloudbees;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseRefreshToken {
    /// Refresh token expiration types, one of: expiring, non-expiring
    expiration_type: String,
    /// Refresh token rotation types, one of: rotating, non-rotating
    rotation_type: String,
    /// Period in seconds where the previous refresh token can be exchanged without triggering breach detection
    leeway: i32,
    /// Period (in seconds) for which refresh tokens will remain valid.
    token_lifetime: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseSamlp {
    recipient: String,
    destination: String,
    issuer: String,
    #[serde(rename = "authnContextClassRef")]
    authn_context_class_ref: String,
    #[serde(rename = "digestAlgorithm")]
    digest_algorithm: String,
    audience: String,
    #[serde(rename = "lifetimeInSeconds")]
    lifetime_in_seconds: i32,
    #[serde(rename = "signatureAlgorithm")]
    signature_algorithm: String,
    #[serde(rename = "nameIdentifierFormat")]
    name_identifier_format: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseSalesforceSandboxApi {
    /// Consumer Key assigned by Salesforce to the Connected App.
    clientid: String,
    /// Community name.
    #[serde(rename = "communityName")]
    community_name: String,
    /// Community url section.
    community_url_section: String,
    /// Name of the property in the user object that maps to a Salesforce username. e.g. `email`.
    principal: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyNativeSocialLogin;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseSalesforceApi {
    /// Name of the property in the user object that maps to a Salesforce username. e.g. `email`.
    principal: String,
    /// Community url section.
    community_url_section: String,
    /// Community name.
    #[serde(rename = "communityName")]
    community_name: String,
    /// Consumer Key assigned by Salesforce to the Connected App.
    clientid: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseDropbox;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyOffice365 {
    /// Optional Auth0 database connection for testing an already-configured Office 365 tenant.
    connection: String,
    /// Your Office 365 domain name. e.g. `acme-org.com`.
    domain: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseWsfed;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseJwtConfiguration {
    /// Algorithm used to sign JWTs. Can be `HS256` or `RS256`.
    alg: String,
    /// Number of seconds the JWT will be valid for (affects `exp` claim).
    lifetime_in_seconds: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseSalesforceSandboxApi {
    /// Consumer Key assigned by Salesforce to the Connected App.
    clientid: String,
    /// Community name.
    #[serde(rename = "communityName")]
    community_name: String,
    /// Name of the property in the user object that maps to a Salesforce username. e.g. `email`.
    principal: String,
    /// Community url section.
    community_url_section: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseAddons;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseAws {
    /// AWS principal ARN, e.g. `arn:aws:iam::010616021751:saml-provider/idpname`
    principal: String,
    /// AWS role ARN, e.g. `arn:aws:iam::010616021751:role/foo`
    role: String,
    /// AWS token lifetime in seconds
    lifetime_in_seconds: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyLayer {
    /// Authentication Key identifier used to sign the Layer token.
    #[serde(rename = "keyId")]
    key_id: String,
    /// Name of the property used as the unique user id in Layer. If not specified `user_id` is used.
    principal: String,
    /// Provider ID of your Layer account
    #[serde(rename = "providerId")]
    provider_id: String,
    /// Optional expiration in minutes for the generated token. Defaults to 5 minutes.
    expiration: i32,
    /// Private key for signing the Layer token.
    #[serde(rename = "privateKey")]
    private_key: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseAzureSb {
    /// Your Azure Service Bus namespace. Usually the first segment of your Service Bus URL (e.g. `https://acme-org.servicebus.windows.net` would be `acme-org`).
    namespace: String,
    /// Entity you want to request a token for. e.g. `my-queue`.'
    #[serde(rename = "entityPath")]
    entity_path: String,
    /// Your shared access policy name defined in your Service Bus entity.
    #[serde(rename = "sasKeyName")]
    sas_key_name: String,
    /// Primary Key associated with your shared access policy.
    #[serde(rename = "sasKey")]
    sas_key: String,
    /// Optional expiration in minutes for the generated token. Defaults to 5 minutes.
    expiration: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseClientMetadata;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseSapApi {
    /// NameID element of the Subject which can be used to express the user's identity. Defaults to `urn:oasis:names:tc:SAML:1.1:nameid-format:unspecified`.
    #[serde(rename = "nameIdentifierFormat")]
    name_identifier_format: String,
    /// Name of the property in the user object that maps to a SAP username. e.g. `email`.
    #[serde(rename = "usernameAttribute")]
    username_attribute: String,
    /// Your SAP OData server OAuth2 token endpoint URL.
    #[serde(rename = "tokenEndpointUrl")]
    token_endpoint_url: String,
    /// If activated in the OAuth 2.0 client configuration (transaction SOAUTH2) the SAML attribute client_id must be set and equal the client_id form parameter of the access token request.
    clientid: String,
    /// Requested scope for SAP APIs.
    scope: String,
    /// Service account password to use to authenticate API calls to the token endpoint.
    #[serde(rename = "servicePassword")]
    service_password: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseSlack {
    /// Slack team name.
    team: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseEchosign {
    /// Your custom domain found in your EchoSign URL. e.g. `https://acme-org.echosign.com` would be `acme-org`.
    domain: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodySalesforceApi {
    /// Consumer Key assigned by Salesforce to the Connected App.
    clientid: String,
    /// Community name.
    #[serde(rename = "communityName")]
    community_name: String,
    /// Community url section.
    community_url_section: String,
    /// Name of the property in the user object that maps to a Salesforce username. e.g. `email`.
    principal: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseRefreshToken {
    /// Refresh token expiration types, one of: expiring, non-expiring
    expiration_type: String,
    /// Refresh token rotation types, one of: rotating, non-rotating
    rotation_type: String,
    /// Period in seconds where the previous refresh token can be exchanged without triggering breach detection
    leeway: i32,
    /// Period (in seconds) for which refresh tokens will remain valid.
    token_lifetime: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodySharepoint {
    /// Internal SharePoint application URL.
    url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseSpringcm {
    /// SpringCM ACS URL, e.g. `https://na11.springcm.com/atlas/sso/SSOEndpoint.ashx`.
    acsurl: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseSentry {
    /// URL prefix only if running Sentry Community Edition, otherwise leave should be blank.
    base_url: String,
    /// Generated slug for your Sentry organization. Found in your Sentry URL. e.g. `https://sentry.acme.com/acme-org/` would be `acme-org`.
    org_slug: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyMobile;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseAddons;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseCloudbees;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseSalesforce {
    /// Arbitrary logical URL that identifies the Saleforce resource. e.g. `https://acme-org.com`.
    entity_id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyEncryptionKey {
    /// Certificate subject
    subject: String,
    /// Encryption certificate
    cert: String,
    /// Encryption public key
    #[serde(rename = "pub")]
    public: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseZoom {
    /// Zoom account name usually first segment of your Zoom URL, e.g. `https://acme-org.zoom.us` would be `acme-org`.
    account: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseConcur;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyDropbox;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseLayer {
    /// Name of the property used as the unique user id in Layer. If not specified `user_id` is used.
    principal: String,
    /// Authentication Key identifier used to sign the Layer token.
    #[serde(rename = "keyId")]
    key_id: String,
    /// Provider ID of your Layer account
    #[serde(rename = "providerId")]
    provider_id: String,
    /// Private key for signing the Layer token.
    #[serde(rename = "privateKey")]
    private_key: String,
    /// Optional expiration in minutes for the generated token. Defaults to 5 minutes.
    expiration: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseIos {
    /// Assigned by developer to the app as its unique identifier inside the store. Usually this is a reverse domain plus the app name, e.g. `com.you.MyApp`.
    app_bundle_identifier: String,
    /// Identifier assigned to the Apple account that signs and uploads the app to the store.
    team_id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyAzureSb {
    /// Your Azure Service Bus namespace. Usually the first segment of your Service Bus URL (e.g. `https://acme-org.servicebus.windows.net` would be `acme-org`).
    namespace: String,
    /// Entity you want to request a token for. e.g. `my-queue`.'
    #[serde(rename = "entityPath")]
    entity_path: String,
    /// Your shared access policy name defined in your Service Bus entity.
    #[serde(rename = "sasKeyName")]
    sas_key_name: String,
    /// Optional expiration in minutes for the generated token. Defaults to 5 minutes.
    expiration: i32,
    /// Primary Key associated with your shared access policy.
    #[serde(rename = "sasKey")]
    sas_key: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseSpringcm {
    /// SpringCM ACS URL, e.g. `https://na11.springcm.com/atlas/sso/SSOEndpoint.ashx`.
    acsurl: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyNewrelic {
    /// Your New Relic Account ID found in your New Relic URL after the `/accounts/` path. e.g. `https://rpm.newrelic.com/accounts/123456/query` would be `123456`.
    account: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseConcur;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseZoom {
    /// Zoom account name usually first segment of your Zoom URL, e.g. `https://acme-org.zoom.us` would be `acme-org`.
    account: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseOffice365 {
    /// Your Office 365 domain name. e.g. `acme-org.com`.
    domain: String,
    /// Optional Auth0 database connection for testing an already-configured Office 365 tenant.
    connection: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseSsoIntegration {
    /// SSO integration name
    name: String,
    /// SSO integration version installed
    version: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseAws {
    /// AWS token lifetime in seconds
    lifetime_in_seconds: i32,
    /// AWS principal ARN, e.g. `arn:aws:iam::010616021751:saml-provider/idpname`
    principal: String,
    /// AWS role ARN, e.g. `arn:aws:iam::010616021751:role/foo`
    role: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseBox;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseIos {
    /// Assigned by developer to the app as its unique identifier inside the store. Usually this is a reverse domain plus the app name, e.g. `com.you.MyApp`.
    app_bundle_identifier: String,
    /// Identifier assigned to the Apple account that signs and uploads the app to the store.
    team_id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseSalesforceApi {
    /// Community name.
    #[serde(rename = "communityName")]
    community_name: String,
    /// Consumer Key assigned by Salesforce to the Connected App.
    clientid: String,
    /// Name of the property in the user object that maps to a Salesforce username. e.g. `email`.
    principal: String,
    /// Community url section.
    community_url_section: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseIos {
    /// Assigned by developer to the app as its unique identifier inside the store. Usually this is a reverse domain plus the app name, e.g. `com.you.MyApp`.
    app_bundle_identifier: String,
    /// Identifier assigned to the Apple account that signs and uploads the app to the store.
    team_id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyAzureSb {
    /// Optional expiration in minutes for the generated token. Defaults to 5 minutes.
    expiration: i32,
    /// Your Azure Service Bus namespace. Usually the first segment of your Service Bus URL (e.g. `https://acme-org.servicebus.windows.net` would be `acme-org`).
    namespace: String,
    /// Entity you want to request a token for. e.g. `my-queue`.'
    #[serde(rename = "entityPath")]
    entity_path: String,
    /// Primary Key associated with your shared access policy.
    #[serde(rename = "sasKey")]
    sas_key: String,
    /// Your shared access policy name defined in your Service Bus entity.
    #[serde(rename = "sasKeyName")]
    sas_key_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodySamlp {
    #[serde(rename = "nameIdentifierFormat")]
    name_identifier_format: String,
    #[serde(rename = "digestAlgorithm")]
    digest_algorithm: String,
    audience: String,
    #[serde(rename = "signatureAlgorithm")]
    signature_algorithm: String,
    destination: String,
    recipient: String,
    issuer: String,
    #[serde(rename = "authnContextClassRef")]
    authn_context_class_ref: String,
    #[serde(rename = "lifetimeInSeconds")]
    lifetime_in_seconds: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponse {
    /// ID of this client.
    client_id: String,
    /// Name of this client (min length: 1 character, does not allow `<` or `>`).
    name: String,
    /// Free text description of this client (max length: 140 characters).
    description: String,
    /// Name of the tenant this client belongs to.
    tenant: String,
    /// Addons enabled for this client and their associated configurations.
    addons: GetClientsByIdResponseAddons,
    /// URL of the location in your site where the cross origin verification takes place for the cross-origin auth flow when performing Auth in your own domain instead of Auth0 hosted login page.
    cross_origin_loc: String,
    /// The content (HTML, CSS, JS) of the custom login page. (Used on Previews)
    custom_login_page_preview: String,
    /// Type of client used to determine which settings are applicable. Can be `spa`, `native`, `non_interactive`, or `regular_web`.
    app_type: String,
    /// Defines the requested authentication method for the token endpoint. Can be `none` (public client without a client secret), `client_secret_post` (client uses HTTP POST parameters), or `client_secret_basic` (client uses HTTP Basic).
    token_endpoint_auth_method: String,
    /// Initiate login uri, must be https and cannot contain a fragment
    initiate_login_uri: String,
    /// Configuration related to JWTs for the client.
    jwt_configuration: GetClientsByIdResponseJwtConfiguration,
    /// Configure native social settings
    native_social_login: GetClientsByIdResponseNativeSocialLogin,
    /// URL of the logo to display for this client. Recommended size is 150x150 pixels.
    logo_uri: String,
    /// HTML form template to be used for WS-Federation.
    form_template: String,
    /// Metadata associated with the client, in the form of an object with string values (max 255 chars).  Maximum of 10 metadata properties allowed.
    client_metadata: GetClientsByIdResponseClientMetadata,
    /// Additional configuration for native mobile apps.
    mobile: GetClientsByIdResponseMobile,
    /// Refresh token configuration
    refresh_token: GetClientsByIdResponseRefreshToken,
    /// The content (HTML, CSS, JS) of the custom login page.
    custom_login_page: String,
    /// Client secret (which you must not make public).
    client_secret: String,
    /// Encryption used for WsFed responses with this client.
    encryption_key: GetClientsByIdResponseEncryptionKey,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodySalesforce {
    /// Arbitrary logical URL that identifies the Saleforce resource. e.g. `https://acme-org.com`.
    entity_id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyAndroid {
    /// App package name found in AndroidManifest.xml.
    app_package_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseCloudbees;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseZendesk {
    /// Zendesk account name usually first segment in your Zendesk URL. e.g. `https://acme-org.zendesk.com` would be `acme-org`.
    #[serde(rename = "accountName")]
    account_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponse {
    /// Refresh token configuration
    refresh_token: PostRotateSecretResponseRefreshToken,
    /// Addons enabled for this client and their associated configurations.
    addons: PostRotateSecretResponseAddons,
    /// Name of the tenant this client belongs to.
    tenant: String,
    /// URL of the location in your site where the cross origin verification takes place for the cross-origin auth flow when performing Auth in your own domain instead of Auth0 hosted login page.
    cross_origin_loc: String,
    /// URL of the logo to display for this client. Recommended size is 150x150 pixels.
    logo_uri: String,
    /// The content (HTML, CSS, JS) of the custom login page.
    custom_login_page: String,
    /// Additional configuration for native mobile apps.
    mobile: PostRotateSecretResponseMobile,
    /// Configuration related to JWTs for the client.
    jwt_configuration: PostRotateSecretResponseJwtConfiguration,
    /// The content (HTML, CSS, JS) of the custom login page. (Used on Previews)
    custom_login_page_preview: String,
    /// Free text description of this client (max length: 140 characters).
    description: String,
    /// ID of this client.
    client_id: String,
    /// Name of this client (min length: 1 character, does not allow `<` or `>`).
    name: String,
    /// Client secret (which you must not make public).
    client_secret: String,
    /// Initiate login uri, must be https and cannot contain a fragment
    initiate_login_uri: String,
    /// HTML form template to be used for WS-Federation.
    form_template: String,
    /// Encryption used for WsFed responses with this client.
    encryption_key: PostRotateSecretResponseEncryptionKey,
    /// Defines the requested authentication method for the token endpoint. Can be `none` (public client without a client secret), `client_secret_post` (client uses HTTP POST parameters), or `client_secret_basic` (client uses HTTP Basic).
    token_endpoint_auth_method: String,
    /// Configure native social settings
    native_social_login: PostRotateSecretResponseNativeSocialLogin,
    /// Type of client used to determine which settings are applicable. Can be `spa`, `native`, `non_interactive`, or `regular_web`.
    app_type: String,
    /// Metadata associated with the client, in the form of an object with string values (max 255 chars).  Maximum of 10 metadata properties allowed.
    client_metadata: PostRotateSecretResponseClientMetadata,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyScopes;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseClientMetadata;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseZoom {
    /// Zoom account name usually first segment of your Zoom URL, e.g. `https://acme-org.zoom.us` would be `acme-org`.
    account: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseAzureBlob {
    /// Container to request a token for. e.g. `my-container`.
    #[serde(rename = "containerName")]
    container_name: String,
    /// Your Azure storage account name. Usually first segment in your Azure storage URL. e.g. `https://acme-org.blob.core.windows.net` would be the account name `acme-org`.
    #[serde(rename = "accountName")]
    account_name: String,
    /// Entity to request a token for. e.g. `my-blob`. If blank the computed SAS will apply to the entire storage container.
    #[serde(rename = "blobName")]
    blob_name: String,
    /// Expiration in minutes for the generated token (default of 5 minutes).
    expiration: i32,
    /// Access key associated with this storage account.
    #[serde(rename = "storageAccessKey")]
    storage_access_key: String,
    /// Shared access policy identifier defined in your storage account resource.
    #[serde(rename = "signedIdentifier")]
    signed_identifier: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyEgnyte {
    /// Your custom domain found in your Egnyte URL. e.g. `https://acme-org.egnyte.com` would be `acme-org`.
    domain: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseBox;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseEncryptionKey {
    /// Encryption Public RSA Key.
    #[serde(rename = "pub")]
    public: String,
    /// Encryption certificate for public key in X.590 (.CER) format.
    cert: String,
    /// Encryption certificate name for this certificate in the format `/CN={domain}`.
    subject: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseAzureSb {
    /// Your Azure Service Bus namespace. Usually the first segment of your Service Bus URL (e.g. `https://acme-org.servicebus.windows.net` would be `acme-org`).
    namespace: String,
    /// Your shared access policy name defined in your Service Bus entity.
    #[serde(rename = "sasKeyName")]
    sas_key_name: String,
    /// Primary Key associated with your shared access policy.
    #[serde(rename = "sasKey")]
    sas_key: String,
    /// Optional expiration in minutes for the generated token. Defaults to 5 minutes.
    expiration: i32,
    /// Entity you want to request a token for. e.g. `my-queue`.'
    #[serde(rename = "entityPath")]
    entity_path: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseNativeSocialLogin;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseLayer {
    /// Optional expiration in minutes for the generated token. Defaults to 5 minutes.
    expiration: i32,
    /// Private key for signing the Layer token.
    #[serde(rename = "privateKey")]
    private_key: String,
    /// Provider ID of your Layer account
    #[serde(rename = "providerId")]
    provider_id: String,
    /// Authentication Key identifier used to sign the Layer token.
    #[serde(rename = "keyId")]
    key_id: String,
    /// Name of the property used as the unique user id in Layer. If not specified `user_id` is used.
    principal: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseSharepoint {
    /// Internal SharePoint application URL.
    url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseCloudbees;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseClientMetadata;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyAws {
    /// AWS token lifetime in seconds
    lifetime_in_seconds: i32,
    /// AWS role ARN, e.g. `arn:aws:iam::010616021751:role/foo`
    role: String,
    /// AWS principal ARN, e.g. `arn:aws:iam::010616021751:saml-provider/idpname`
    principal: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyMobile;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseFirebase {
    /// ID of the Service Account you have created (shown as `client_email` in the generated JSON file, SDK v3+ tokens only).
    client_email: String,
    /// Google Firebase Secret. (SDK 2 only).
    secret: String,
    /// Optional ID of the private key to obtain kid header in the issued token (SDK v3+ tokens only).
    private_key_id: String,
    /// Optional expiration in seconds for the generated token. Defaults to 3600 seconds (SDK v3+ tokens only).
    lifetime_in_seconds: i32,
    /// Private Key for signing the token (SDK v3+ tokens only).
    private_key: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseNewrelic {
    /// Your New Relic Account ID found in your New Relic URL after the `/accounts/` path. e.g. `https://rpm.newrelic.com/accounts/123456/query` would be `123456`.
    account: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseAddons;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseSamlp {
    #[serde(rename = "nameIdentifierFormat")]
    name_identifier_format: String,
    destination: String,
    issuer: String,
    recipient: String,
    #[serde(rename = "authnContextClassRef")]
    authn_context_class_ref: String,
    #[serde(rename = "signatureAlgorithm")]
    signature_algorithm: String,
    #[serde(rename = "lifetimeInSeconds")]
    lifetime_in_seconds: i32,
    #[serde(rename = "digestAlgorithm")]
    digest_algorithm: String,
    audience: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseDropbox;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseWams {
    /// Your master key for Windows Azure Mobile Services.
    masterkey: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseSharepoint {
    /// Internal SharePoint application URL.
    url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseSsoIntegration {
    /// SSO integration version installed
    version: String,
    /// SSO integration name
    name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseAndroid {
    /// App package name found in AndroidManifest.xml.
    app_package_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseRefreshToken {
    /// Refresh token rotation types, one of: rotating, non-rotating
    rotation_type: String,
    /// Period (in seconds) for which refresh tokens will remain valid.
    token_lifetime: i32,
    /// Refresh token expiration types, one of: expiring, non-expiring
    expiration_type: String,
    /// Period in seconds where the previous refresh token can be exchanged without triggering breach detection
    leeway: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseMscrm {
    /// Microsoft Dynamics CRM application URL.
    url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseSpringcm {
    /// SpringCM ACS URL, e.g. `https://na11.springcm.com/atlas/sso/SSOEndpoint.ashx`.
    acsurl: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodySalesforceSandboxApi {
    /// Community url section.
    community_url_section: String,
    /// Consumer Key assigned by Salesforce to the Connected App.
    clientid: String,
    /// Name of the property in the user object that maps to a Salesforce username. e.g. `email`.
    principal: String,
    /// Community name.
    #[serde(rename = "communityName")]
    community_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseEncryptionKey {
    /// Encryption certificate for public key in X.590 (.CER) format.
    cert: String,
    /// Encryption Public RSA Key.
    #[serde(rename = "pub")]
    public: String,
    /// Encryption certificate name for this certificate in the format `/CN={domain}`.
    subject: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseAddons;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodySapApi {
    /// Service account password to use to authenticate API calls to the token endpoint.
    #[serde(rename = "servicePassword")]
    service_password: String,
    /// Requested scope for SAP APIs.
    scope: String,
    /// NameID element of the Subject which can be used to express the user's identity. Defaults to `urn:oasis:names:tc:SAML:1.1:nameid-format:unspecified`.
    #[serde(rename = "nameIdentifierFormat")]
    name_identifier_format: String,
    /// If activated in the OAuth 2.0 client configuration (transaction SOAUTH2) the SAML attribute client_id must be set and equal the client_id form parameter of the access token request.
    clientid: String,
    /// Your SAP OData server OAuth2 token endpoint URL.
    #[serde(rename = "tokenEndpointUrl")]
    token_endpoint_url: String,
    /// Name of the property in the user object that maps to a SAP username. e.g. `email`.
    #[serde(rename = "usernameAttribute")]
    username_attribute: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseLayer {
    /// Optional expiration in minutes for the generated token. Defaults to 5 minutes.
    expiration: i32,
    /// Provider ID of your Layer account
    #[serde(rename = "providerId")]
    provider_id: String,
    /// Name of the property used as the unique user id in Layer. If not specified `user_id` is used.
    principal: String,
    /// Private key for signing the Layer token.
    #[serde(rename = "privateKey")]
    private_key: String,
    /// Authentication Key identifier used to sign the Layer token.
    #[serde(rename = "keyId")]
    key_id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseWams {
    /// Your master key for Windows Azure Mobile Services.
    masterkey: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseMscrm {
    /// Microsoft Dynamics CRM application URL.
    url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseDropbox;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyEncryptionKey {
    /// Encryption Public RSA Key.
    #[serde(rename = "pub")]
    public: String,
    /// Encryption certificate for public key in X.590 (.CER) format.
    cert: String,
    /// Encryption certificate name for this certificate in the format `/CN={domain}`.
    subject: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodySamlp {
    recipient: String,
    #[serde(rename = "digestAlgorithm")]
    digest_algorithm: String,
    #[serde(rename = "lifetimeInSeconds")]
    lifetime_in_seconds: i32,
    #[serde(rename = "signatureAlgorithm")]
    signature_algorithm: String,
    issuer: String,
    #[serde(rename = "authnContextClassRef")]
    authn_context_class_ref: String,
    destination: String,
    #[serde(rename = "nameIdentifierFormat")]
    name_identifier_format: String,
    audience: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseRms {
    /// URL of your Rights Management Server. It can be internal or external, but users will have to be able to reach it.
    url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseEgnyte {
    /// Your custom domain found in your Egnyte URL. e.g. `https://acme-org.egnyte.com` would be `acme-org`.
    domain: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyWams {
    /// Your master key for Windows Azure Mobile Services.
    masterkey: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyZoom {
    /// Zoom account name usually first segment of your Zoom URL, e.g. `https://acme-org.zoom.us` would be `acme-org`.
    account: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseEncryptionKey {
    /// Encryption Public RSA Key.
    #[serde(rename = "pub")]
    public: String,
    /// Encryption certificate for public key in X.590 (.CER) format.
    cert: String,
    /// Encryption certificate name for this certificate in the format `/CN={domain}`.
    subject: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseSentry {
    /// URL prefix only if running Sentry Community Edition, otherwise leave should be blank.
    base_url: String,
    /// Generated slug for your Sentry organization. Found in your Sentry URL. e.g. `https://sentry.acme.com/acme-org/` would be `acme-org`.
    org_slug: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseSlack {
    /// Slack team name.
    team: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseRms {
    /// URL of your Rights Management Server. It can be internal or external, but users will have to be able to reach it.
    url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseSamlp {
    destination: String,
    #[serde(rename = "digestAlgorithm")]
    digest_algorithm: String,
    #[serde(rename = "lifetimeInSeconds")]
    lifetime_in_seconds: i32,
    #[serde(rename = "signatureAlgorithm")]
    signature_algorithm: String,
    audience: String,
    #[serde(rename = "authnContextClassRef")]
    authn_context_class_ref: String,
    recipient: String,
    issuer: String,
    #[serde(rename = "nameIdentifierFormat")]
    name_identifier_format: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyConcur;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseScopes;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseSalesforce {
    /// Arbitrary logical URL that identifies the Saleforce resource. e.g. `https://acme-org.com`.
    entity_id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyBox;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseSlack {
    /// Slack team name.
    team: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseEgnyte {
    /// Your custom domain found in your Egnyte URL. e.g. `https://acme-org.egnyte.com` would be `acme-org`.
    domain: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseEgnyte {
    /// Your custom domain found in your Egnyte URL. e.g. `https://acme-org.egnyte.com` would be `acme-org`.
    domain: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseJwtConfiguration {
    /// Number of seconds the JWT will be valid for (affects `exp` claim).
    lifetime_in_seconds: i32,
    /// Algorithm used to sign JWTs. Can be `HS256` or `RS256`.
    alg: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseEchosign {
    /// Your custom domain found in your EchoSign URL. e.g. `https://acme-org.echosign.com` would be `acme-org`.
    domain: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseNativeSocialLogin;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyMscrm {
    /// Microsoft Dynamics CRM application URL.
    url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyDropbox;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseMscrm {
    /// Microsoft Dynamics CRM application URL.
    url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseApple;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodySalesforceApi {
    /// Community url section.
    community_url_section: String,
    /// Consumer Key assigned by Salesforce to the Connected App.
    clientid: String,
    /// Name of the property in the user object that maps to a Salesforce username. e.g. `email`.
    principal: String,
    /// Community name.
    #[serde(rename = "communityName")]
    community_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseBox;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseEchosign {
    /// Your custom domain found in your EchoSign URL. e.g. `https://acme-org.echosign.com` would be `acme-org`.
    domain: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodySentry {
    /// Generated slug for your Sentry organization. Found in your Sentry URL. e.g. `https://sentry.acme.com/acme-org/` would be `acme-org`.
    org_slug: String,
    /// URL prefix only if running Sentry Community Edition, otherwise leave should be blank.
    base_url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseFacebook;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseZendesk {
    /// Zendesk account name usually first segment in your Zendesk URL. e.g. `https://acme-org.zendesk.com` would be `acme-org`.
    #[serde(rename = "accountName")]
    account_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseNativeSocialLogin;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseFacebook;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseFacebook;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseSapApi {
    /// Your SAP OData server OAuth2 token endpoint URL.
    #[serde(rename = "tokenEndpointUrl")]
    token_endpoint_url: String,
    /// Name of the property in the user object that maps to a SAP username. e.g. `email`.
    #[serde(rename = "usernameAttribute")]
    username_attribute: String,
    /// Requested scope for SAP APIs.
    scope: String,
    /// NameID element of the Subject which can be used to express the user's identity. Defaults to `urn:oasis:names:tc:SAML:1.1:nameid-format:unspecified`.
    #[serde(rename = "nameIdentifierFormat")]
    name_identifier_format: String,
    /// If activated in the OAuth 2.0 client configuration (transaction SOAUTH2) the SAML attribute client_id must be set and equal the client_id form parameter of the access token request.
    clientid: String,
    /// Service account password to use to authenticate API calls to the token endpoint.
    #[serde(rename = "servicePassword")]
    service_password: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseNewrelic {
    /// Your New Relic Account ID found in your New Relic URL after the `/accounts/` path. e.g. `https://rpm.newrelic.com/accounts/123456/query` would be `123456`.
    account: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseDropbox;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseSlack {
    /// Slack team name.
    team: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseWams {
    /// Your master key for Windows Azure Mobile Services.
    masterkey: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseConcur;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseWsfed;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseMappings;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodySlack {
    /// Slack team name.
    team: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyNewrelic {
    /// Your New Relic Account ID found in your New Relic URL after the `/accounts/` path. e.g. `https://rpm.newrelic.com/accounts/123456/query` would be `123456`.
    account: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseSamlp {
    #[serde(rename = "nameIdentifierFormat")]
    name_identifier_format: String,
    recipient: String,
    #[serde(rename = "signatureAlgorithm")]
    signature_algorithm: String,
    #[serde(rename = "lifetimeInSeconds")]
    lifetime_in_seconds: i32,
    #[serde(rename = "authnContextClassRef")]
    authn_context_class_ref: String,
    destination: String,
    audience: String,
    #[serde(rename = "digestAlgorithm")]
    digest_algorithm: String,
    issuer: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseFacebook;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseZoom {
    /// Zoom account name usually first segment of your Zoom URL, e.g. `https://acme-org.zoom.us` would be `acme-org`.
    account: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseLayer {
    /// Name of the property used as the unique user id in Layer. If not specified `user_id` is used.
    principal: String,
    /// Private key for signing the Layer token.
    #[serde(rename = "privateKey")]
    private_key: String,
    /// Provider ID of your Layer account
    #[serde(rename = "providerId")]
    provider_id: String,
    /// Authentication Key identifier used to sign the Layer token.
    #[serde(rename = "keyId")]
    key_id: String,
    /// Optional expiration in minutes for the generated token. Defaults to 5 minutes.
    expiration: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseAndroid {
    /// App package name found in AndroidManifest.xml.
    app_package_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyRms {
    /// URL of your Rights Management Server. It can be internal or external, but users will have to be able to reach it.
    url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseSentry {
    /// Generated slug for your Sentry organization. Found in your Sentry URL. e.g. `https://sentry.acme.com/acme-org/` would be `acme-org`.
    org_slug: String,
    /// URL prefix only if running Sentry Community Edition, otherwise leave should be blank.
    base_url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodySpringcm {
    /// SpringCM ACS URL, e.g. `https://na11.springcm.com/atlas/sso/SSOEndpoint.ashx`.
    acsurl: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseMappings;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyIos {
    /// Assigned by the developer to the app as its unique identifier inside the store, usually is a reverse domain plus the app name: <code>com.you.MyApp</code>
    app_bundle_identifier: String,
    /// Identifier assigned to the account that signs and upload the app to the store
    team_id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodySlack {
    /// Slack team name.
    team: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseMobile;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseAzureSb {
    /// Your shared access policy name defined in your Service Bus entity.
    #[serde(rename = "sasKeyName")]
    sas_key_name: String,
    /// Optional expiration in minutes for the generated token. Defaults to 5 minutes.
    expiration: i32,
    /// Your Azure Service Bus namespace. Usually the first segment of your Service Bus URL (e.g. `https://acme-org.servicebus.windows.net` would be `acme-org`).
    namespace: String,
    /// Primary Key associated with your shared access policy.
    #[serde(rename = "sasKey")]
    sas_key: String,
    /// Entity you want to request a token for. e.g. `my-queue`.'
    #[serde(rename = "entityPath")]
    entity_path: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyWsfed;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodySalesforceSandboxApi {
    /// Consumer Key assigned by Salesforce to the Connected App.
    clientid: String,
    /// Name of the property in the user object that maps to a Salesforce username. e.g. `email`.
    principal: String,
    /// Community url section.
    community_url_section: String,
    /// Community name.
    #[serde(rename = "communityName")]
    community_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseAndroid {
    /// App package name found in AndroidManifest.xml.
    app_package_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseSpringcm {
    /// SpringCM ACS URL, e.g. `https://na11.springcm.com/atlas/sso/SSOEndpoint.ashx`.
    acsurl: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyFirebase {
    /// ID of the Service Account you have created (shown as `client_email` in the generated JSON file, SDK v3+ tokens only).
    client_email: String,
    /// Optional expiration in seconds for the generated token. Defaults to 3600 seconds (SDK v3+ tokens only).
    lifetime_in_seconds: i32,
    /// Optional ID of the private key to obtain kid header in the issued token (SDK v3+ tokens only).
    private_key_id: String,
    /// Google Firebase Secret. (SDK 2 only).
    secret: String,
    /// Private Key for signing the token (SDK v3+ tokens only).
    private_key: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdBodyAndroid {
    /// Application package name found in <code>AndroidManifest.xml</code>
    app_package_name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseAzureBlob {
    /// Container to request a token for. e.g. `my-container`.
    #[serde(rename = "containerName")]
    container_name: String,
    /// Expiration in minutes for the generated token (default of 5 minutes).
    expiration: i32,
    /// Entity to request a token for. e.g. `my-blob`. If blank the computed SAS will apply to the entire storage container.
    #[serde(rename = "blobName")]
    blob_name: String,
    /// Access key associated with this storage account.
    #[serde(rename = "storageAccessKey")]
    storage_access_key: String,
    /// Your Azure storage account name. Usually first segment in your Azure storage URL. e.g. `https://acme-org.blob.core.windows.net` would be the account name `acme-org`.
    #[serde(rename = "accountName")]
    account_name: String,
    /// Shared access policy identifier defined in your storage account resource.
    #[serde(rename = "signedIdentifier")]
    signed_identifier: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseEncryptionKey {
    /// Encryption Public RSA Key.
    #[serde(rename = "pub")]
    public: String,
    /// Encryption certificate name for this certificate in the format `/CN={domain}`.
    subject: String,
    /// Encryption certificate for public key in X.590 (.CER) format.
    cert: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseOffice365 {
    /// Optional Auth0 database connection for testing an already-configured Office 365 tenant.
    connection: String,
    /// Your Office 365 domain name. e.g. `acme-org.com`.
    domain: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseRms {
    /// URL of your Rights Management Server. It can be internal or external, but users will have to be able to reach it.
    url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseScopes;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseFacebook;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRotateSecretResponseMscrm {
    /// Microsoft Dynamics CRM application URL.
    url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyEchosign {
    /// Your custom domain found in your EchoSign URL. e.g. `https://acme-org.echosign.com` would be `acme-org`.
    domain: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsResponseWams {
    /// Your master key for Windows Azure Mobile Services.
    masterkey: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsBodyZoom {
    /// Zoom account name usually first segment of your Zoom URL, e.g. `https://acme-org.zoom.us` would be `acme-org`.
    account: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetClientsByIdResponseSsoIntegration {
    /// SSO integration version installed
    version: String,
    /// SSO integration name
    name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseSamlp {
    recipient: String,
    #[serde(rename = "digestAlgorithm")]
    digest_algorithm: String,
    issuer: String,
    #[serde(rename = "authnContextClassRef")]
    authn_context_class_ref: String,
    #[serde(rename = "signatureAlgorithm")]
    signature_algorithm: String,
    #[serde(rename = "lifetimeInSeconds")]
    lifetime_in_seconds: i32,
    audience: String,
    #[serde(rename = "nameIdentifierFormat")]
    name_identifier_format: String,
    destination: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostClientsResponseFirebase {
    /// Optional ID of the private key to obtain kid header in the issued token (SDK v3+ tokens only).
    private_key_id: String,
    /// Google Firebase Secret. (SDK 2 only).
    secret: String,
    /// Private Key for signing the token (SDK v3+ tokens only).
    private_key: String,
    /// ID of the Service Account you have created (shown as `client_email` in the generated JSON file, SDK v3+ tokens only).
    client_email: String,
    /// Optional expiration in seconds for the generated token. Defaults to 3600 seconds (SDK v3+ tokens only).
    lifetime_in_seconds: i32,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchClientsByIdResponseAddons;
}

mod resource_servers {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetResourceServersByIdResponse {
    /// Secret used to sign tokens when using symmetric algorithms (HS256).
    signing_secret: String,
    client: GetResourceServersByIdResponseClient,
    /// Unique identifier for the API used as the audience parameter on authorization calls. Can not be changed once set.
    identifier: String,
    /// Dialect of access tokens that should be issued. Can be `access_token` or `access_token_authz` (includes permissions).
    token_dialect: String,
    /// Expiration value (in seconds) for access tokens issued for this API from the token endpoint.
    token_lifetime: i32,
    /// Algorithm used to sign JWTs. Can be `HS256` or `RS256`.
    signing_alg: String,
    /// ID of the API (resource server).
    id: String,
    /// Expiration value (in seconds) for access tokens issued for this API via Implicit or Hybrid Flows. Cannot be greater than the `token_lifetime` value.
    token_lifetime_for_web: i32,
    /// Friendly name for this resource server. Can not contain `<` or `>` characters.
    name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetResourceServersResponseClient;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchResourceServersByIdBodyClient;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetResourceServersByIdResponseClient;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchResourceServersByIdBody {
    client: PatchResourceServersByIdBodyClient,
    /// Algorithm used to sign JWTs. Can be `HS256` or `RS256`.
    signing_alg: String,
    /// Expiration value (in seconds) for access tokens issued for this API from the token endpoint.
    token_lifetime: i32,
    /// Secret used to sign tokens when using symmetric algorithms (HS256).
    signing_secret: String,
    /// Dialect of issued access token. Can be `access_token` or `access_token_authz` (includes permissions).
    token_dialect: String,
    /// Friendly name for this resource server. Can not contain `<` or `>` characters.
    name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostResourceServersBody {
    /// Algorithm used to sign JWTs. Can be `HS256` or `RS256`.
    signing_alg: String,
    /// Expiration value (in seconds) for access tokens issued for this API from the token endpoint.
    token_lifetime: i32,
    client: PostResourceServersBodyClient,
    /// Secret used to sign tokens when using symmetric algorithms (HS256).
    signing_secret: String,
    /// Friendly name for this resource server. Can not contain `<` or `>` characters.
    name: String,
    /// Dialect of issued access token. Can be `access_token` or `access_token_authz` (includes permissions). Values can be `access_token` or `access_token_authz` (includes permissions).
    token_dialect: String,
    /// Unique identifier for the API used as the audience parameter on authorization calls. Can not be changed once set.
    identifier: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostResourceServersBodyClient;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetResourceServersResponse {
    /// Algorithm used to sign JWTs. Can be `HS256` or `RS256`.
    signing_alg: String,
    /// Dialect of access tokens that should be issued. Can be `access_token` or `access_token_authz` (includes permissions).
    token_dialect: String,
    client: GetResourceServersResponseClient,
    /// ID of the API (resource server).
    id: String,
    /// Expiration value (in seconds) for access tokens issued for this API from the token endpoint.
    token_lifetime: i32,
    /// Expiration value (in seconds) for access tokens issued for this API via Implicit or Hybrid Flows. Cannot be greater than the `token_lifetime` value.
    token_lifetime_for_web: i32,
    /// Secret used to sign tokens when using symmetric algorithms (HS256).
    signing_secret: String,
    /// Friendly name for this resource server. Can not contain `<` or `>` characters.
    name: String,
    /// Unique identifier for the API used as the audience parameter on authorization calls. Can not be changed once set.
    identifier: String,
  }
}

mod branding {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetBrandingResponsePageBackground;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetUniversalLoginResponse {
    /// The custom page template for the New Universal Login Experience
    body: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchBrandingBodyColors {
    /// Accent color.
    primary: String,
    /// Page Background Color or Gradient.
    /// Property contains either <code>null</code> to unset, a solid color as a string value <code>#FFFFFF</code>, or a gradient as an object.
    ///
    /// <code>
    /// {
    ///   type: 'linear-gradient',
    ///   start: '#FFFFFF',
    ///   end: '#000000',
    ///   angle_deg: 35
    /// }
    /// </code>
    ///
    page_background: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchBrandingBody {
    /// Custom color settings.
    colors: PatchBrandingBodyColors,
    /// URL for the favicon. Must use HTTPS.
    favicon_url: String,
    /// Custom font settings.
    font: PatchBrandingBodyFont,
    /// URL for the logo. Must use HTTPS.
    logo_url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchBrandingResponseFont {
    /// URL for the custom font. The URL must point to a font file and not a stylesheet. Must use HTTPS.
    url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetBrandingResponse {
    /// Custom color settings.
    colors: GetBrandingResponseColors,
    /// URL for the favicon. Must use HTTPS.
    favicon_url: String,
    /// URL for the logo. Must use HTTPS.
    logo_url: String,
    /// Custom font settings.
    font: GetBrandingResponseFont,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchBrandingResponse {
    /// URL for the logo. Must use HTTPS.
    logo_url: String,
    /// Custom color settings.
    colors: PatchBrandingResponseColors,
    /// Custom font settings.
    font: PatchBrandingResponseFont,
    /// URL for the favicon. Must use HTTPS.
    favicon_url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchBrandingResponsePageBackground;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchBrandingBodyPageBackground;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchBrandingResponseColors {
    /// Accent color.
    primary: String,
    /// Page Background Color or Gradient.
    /// Property contains either <code>null</code> to unset, a solid color as a string value <code>#FFFFFF</code>, or a gradient as an object.
    ///
    /// <code>
    /// {
    ///   type: 'linear-gradient',
    ///   start: '#FFFFFF',
    ///   end: '#000000',
    ///   angle_deg: 35
    /// }
    /// </code>
    ///
    page_background: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetBrandingResponseColors {
    /// Accent color.
    primary: String,
    /// Page Background Color or Gradient.
    /// Property contains either <code>null</code> to unset, a solid color as a string value <code>#FFFFFF</code>, or a gradient as an object.
    ///
    /// <code>
    /// {
    ///   type: 'linear-gradient',
    ///   start: '#FFFFFF',
    ///   end: '#000000',
    ///   angle_deg: 35
    /// }
    /// </code>
    ///
    page_background: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetBrandingResponseFont {
    /// URL for the custom font. The URL must point to a font file and not a stylesheet. Must use HTTPS.
    url: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchBrandingBodyFont {
    /// URL for the custom font. The URL must point to a font file and not a stylesheet. Must use HTTPS.
    url: String,
  }
}

mod roles {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetRolesResponse {
    /// Name of the role.
    name: String,
    /// ID for this role.
    id: String,
    /// Description of the role.
    description: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRolesResponse {
    /// ID for this role.
    id: String,
    /// Description of the role.
    description: String,
    /// Name of the role.
    name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRolesBody {
    /// Description of the role.
    description: String,
    /// Name of the role.
    name: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct DeleteRolePermissionAssignmentBody;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRolePermissionAssignmentBody;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetRoleUserResponse {
    /// URL to a picture for this user.
    picture: String,
    /// Email address of this user.
    email: String,
    /// Name of this user.
    name: String,
    /// ID of this user.
    user_id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostRoleUsersBody;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchRolesByIdBody {
    /// Name of this role.
    name: String,
    /// Description of this role.
    description: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PatchRolesByIdResponse {
    /// Description of the role.
    description: String,
    /// Name of the role.
    name: String,
    /// ID for this role.
    id: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetRolesByIdResponse {
    /// Name of the role.
    name: String,
    /// ID for this role.
    id: String,
    /// Description of the role.
    description: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetRolePermissionResponse {
    /// Name of this permission.
    permission_name: String,
    /// Resource server (API) name this permission is for.
    resource_server_name: String,
    /// Resource server (API) identifier that this permission is for.
    resource_server_identifier: String,
    /// Description of this permission.
    description: String,
  }
}

mod keys {
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PutSigningKeysResponse {
    /// Revoked key id
    kid: String,
    /// Revoked key certificate
    cert: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct PostSigningKeysResponse {
    /// Next key certificate
    cert: String,
    /// Next key id
    kid: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetSigningKeysResponse {
    /// The key id of the signing key
    kid: String,
    /// The date and time when the current key was rotated
    current_until: String,
    /// The cert fingerprint
    fingerprint: String,
    /// The public certificate of the signing key in pkcs7 format
    #[serde(rename = "pkcs7")]
    pkcs_7: String,
    /// The cert thumbprint
    thumbprint: String,
    /// The date and time when the key became the current key
    current_since: String,
    /// The public certificate of the signing key
    cert: String,
    /// The date and time when the key was revoked
    revoked_at: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GetSigningKeyResponse {
    /// The date and time when the key was revoked
    revoked_at: String,
    /// The date and time when the current key was rotated
    current_until: String,
    /// The public certificate of the signing key in pkcs7 format
    #[serde(rename = "pkcs7")]
    pkcs_7: String,
    /// The cert fingerprint
    fingerprint: String,
    /// The public certificate of the signing key
    cert: String,
    /// The cert thumbprint
    thumbprint: String,
    /// The date and time when the key became the current key
    current_since: String,
    /// The key id of the signing key
    kid: String,
  }
}
