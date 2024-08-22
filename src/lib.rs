pub use error::{Error, Result};


/// Module for the Tidal API client
pub mod client;
/// Module for authentication and credentials
pub mod auth;
/// Module for accessing and searching the Tidal catalog
pub mod catalogue;
/// Module for user related api functions
pub mod users;

pub mod endpoints;

pub(crate) mod utils;


pub mod error;

#[cfg(test)]
mod tests;
