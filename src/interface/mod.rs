//! Interfaces for clients interacting with the Tidal API
pub mod auth;
pub use auth::{ClientFlow, DeviceFlow, RefreshFlow, UserFlow};
pub mod catalogue;
pub mod users;
