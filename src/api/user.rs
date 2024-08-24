use chrono::{DateTime, NaiveDate, Utc};
use isocountry::CountryCode;
use serde::{Deserialize, Serialize};

use super::{AudioQuality, PaymentType, SubscriptionType};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
  id: u64,
  username: String,
  profile_name: String,
  first_name: String,
  last_name: String,
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

#[derive(Debug, Serialize, Deserialize)]
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
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Subscription {
  #[serde(rename = "type")]
  subscription_type: SubscriptionType,
  offline_grace_period: u64,
}

//{"limit":10,"offset":0,"totalNumberOfItems":2,"items":[{"id":496267582,"name":"Google Pixel 6 Pro","application":{"name":"TIDAL_Android_2.87.0","type":{"name":"AndroidWithBasic"},"service":"TIDAL"},"uniqueKey":"426873ee1f974e54","authorizedForOffline":true,"authorizedForOfflineDate":"2023-01-27T00:35:48.291+0000","lastLogin":"2024-04-13T15:48:40.711+0000","created":"2022-08-19T11:44:48.620+0000","numberOfOfflineAlbums":3,"numberOfOfflinePlaylists":0},{"id":552175926,"name":"Google Pixel 6 Pro","application":{"name":"TIDAL_Android_2.123.0","type":{"name":"AndroidWithBasic"},"service":"TIDAL"},"uniqueKey":"1407211b698380da","authorizedForOffline":true,"authorizedForOfflineDate":"2024-07-21T06:17:13.253+0000","lastLogin":"2024-07-28T13:23:40.922+0000","created":"2024-07-12T15:41:20.915+0000","numberOfOfflineAlbums":0,"numberOfOfflinePlaylists":0}]}
#[derive(Debug, Serialize, Deserialize)]
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
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserClientApplication {
  name: String,
  #[flat_path(path = ["type", "name"])]
  application_type: String,
  service: String,
}
