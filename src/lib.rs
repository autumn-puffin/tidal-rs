//! TIDAL-rs is a wrapper for the Tidal API written in Rust, aiming to provide a simple interface
//! for interacting with the Tidal API.
//!
//! The library is designed to be as flexible as possible, allowing the user to choose how they
//! want to handle authentication, and how they want to store credentials.
//!
//! There are a few clients implimented in the TIDAL-rs, Including the `Client` struct, which is a
//! simple blocking client that impliments all of the traits available, aswell as standalone
//! clients for individual parts of the API, such as the `AuthClient` which impl impliments the
//! `Auth` trait and authentication flows, and the `CatalogueClient` which impliments the
//! `Catalogue` trait and provides access to the Tidal catalogue, such as searching for tracks,
//! albums, and artists.

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
pub use error::{Error, Result};

#[cfg(test)]
mod tests;
