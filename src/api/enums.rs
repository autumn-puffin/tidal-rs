use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AudioQuality {
  Low,
  High,
  Lossless,
  HiRes,
  HiResLossless,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionType {
  Intro,
  Premium,
  PremiumPlus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentType {
  AdyenBoleto,
  AdyenCreditCard,
  AdyenIdeal,
  AdyenSofort,
  ApplePay,
  CashApp,
  Fortumo,
  GooglePlayBilling,
  Paypal,
  Venmo,
  Vivo,
}
