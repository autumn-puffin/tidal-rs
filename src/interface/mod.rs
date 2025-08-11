//! Interfaces for clients interacting with the Tidal API
pub mod auth;
pub use auth::{Auth, ClientFlow, Credentials, DeviceFlow, RefreshFlow, UserFlow};
pub mod catalogue;
pub mod users;
