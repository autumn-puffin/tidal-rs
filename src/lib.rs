pub use error::{Error, Result};


/// Module for the Tidal API client
pub mod client;
/// Module for authentication and credentials
pub mod auth;

pub mod endpoints;

pub(crate) mod utils;


pub mod error;

#[cfg(test)]
mod tests;
