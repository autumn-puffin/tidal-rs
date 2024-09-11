//! # TIDAL-rs
//!
//! TIDAL-rs is a wrapper for the Tidal API written in Rust, aiming to provide a simple interface
//! for interacting with the Tidal API.
//!
//! The library is designed to be as flexible as possible, allowing an application to implement
//! it's own client, or use one or multiple of the clients provided by this library.
//!
//! ## Clients
//!
//! There are a few clients implimented in the TIDAL-rs, Including the [`Client`] struct, which is
//! a simple blocking client that impliments all of the traits available, as well as standalone
//! clients for individual parts of the API, such as the [`AuthClient`] which impliments the
//! [`Auth`] trait and all available authentication flows, and the [`CatalogueClient`] which
//! impliments the [`Catalogue`] trait and provides access to the Tidal catalogue, such as
//! searching for tracks, albums, and artists.
//!
//! Below is a list of the currently available clients:
//! - [`Client`] - A basic all-inclusive blocking client for the Tidal API
//! - [`AuthClient`] - A standalone client for authentication with the Tidal API
//! - [`CatalogueClient`] - A standalone client for accessing the Tidal catalogue
//!  
//! ## Interfaces
//!
//! This library also provides a set of traits to build a custom client with only the functionality
//! you need. These interfaces aim to be as flexible as possible, allowing an implimentor to choose
//! how they handle requests, and how they store and manage credentials and other data.
//!
//! The currently available interfaces are:
//! - [`Auth`] - Manages authentication and credentials
//!   - [`ClientFlow`] - Authenticate using only client credentials
//!   - [`UserFlow`] - Authenticate using user credentials via a redirect
//!   - [`DeviceFlow`] - Authenticate using user credentials via a device code
//!   - [`RefreshFlow`] - Refresh credentials when they expire
//! - [`Catalogue`] - Access the Tidal catalogue and information about tracks, albums, and artists
//! - [`Users`] - Access information about users, including the current user
//!
//! [`Client`]: client/struct.Client.html
//! [`AuthClient`]: client/auth/struct.AuthClient.html
//! [`CatalogueClient`]: client/catalogue/struct.CatalogueClient.html
//! [`Auth`]: interface/auth/trait.Auth.html
//! [`ClientFlow`]: interface/auth/flows/trait.ClientFlow.html
//! [`UserFlow`]: interface/auth/flows/trait.UserFlow.html
//! [`DeviceFlow`]: interface/auth/flows/trait.DeviceFlow.html
//! [`RefreshFlow`]: interface/auth/flows/trait.RefreshFlow.html
//! [`Catalogue`]: interface/catalogue/trait.Catalogue.html
//! [`Users`]: interface/users/trait.Users.html

pub mod api;
pub mod client;
pub mod endpoints;
pub mod interface;

pub(crate) mod utils;

pub mod error;
pub use error::{Error, Result};

#[cfg(test)]
mod tests;
