//! # TIDAL-rs

pub mod api;
pub mod client;
pub mod endpoints;

pub(crate) mod utils;

pub mod error;
pub use error::{Error, Result};

#[cfg(test)]
mod tests;
