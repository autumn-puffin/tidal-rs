use chrono::{DateTime, NaiveDate, Utc};
use isocountry::CountryCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{AudioQuality, PaymentType, SubscriptionType};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
  id: u64,
  username: String,
  profile_name: String,
  first_name: Option<String>,
  last_name: Option<String>,
  email: String,
  email_verified: bool,
  country_code: CountryCode,
  created: DateTime<Utc>,
  newsletter: bool,
  #[serde(rename = "acceptedEULA")]
  accepted_eula: bool,
  date_of_birth: NaiveDate,
  facebook_uid: u64,
  apple_uid: Option<u64>,
  parent_id: u64,
  partner: u64,
  tidal_id: Option<String>, // TODO: Replace with whatever type this is once known
  early_access_program: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSubscription {
  start_date: DateTime<Utc>,
  valid_until: DateTime<Utc>,
  status: String,
  subscription: Subscription,
  highest_sound_quality: AudioQuality,
  premium_access: bool,
  can_get_trial: bool,
  payment_type: PaymentType,
  payment_overdue: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
  #[serde(rename = "type")]
  subscription_type: SubscriptionType,
  offline_grace_period: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserClient {
  id: u64,
  name: String,
  application: UserClientApplication,
  unique_key: String,
  authorized_for_offline: bool,
  authorized_for_offline_date: DateTime<Utc>,
  last_login: DateTime<Utc>,
  created: DateTime<Utc>,
  number_of_offline_albums: u64,
  number_of_offline_playlists: u64,
}

#[serde_flat_path::flat_path]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserClientApplication {
  name: String,
  #[flat_path(path = ["type", "name"])]
  application_type: String,
  service: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Session {
  pub session_id: Uuid,
  pub user_id: u64,
  pub country_code: CountryCode,
  pub channel_id: u64,
  pub partner_id: u64,
  pub client: SessionClient,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SessionClient {
  pub id: u64,
  pub name: String,
  pub authorized_for_offline: bool,
  pub authorized_for_offline_date: Option<u64>,
}
