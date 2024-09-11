//! Tidal API model
//!
//! This module contains the models for data returned from the Tidal API, including catalogue media,
//! page data, various enums, and other data structures.

pub mod paging;
pub use paging::*;
pub mod enums;
pub use enums::*;
pub mod user;
pub use user::*;
pub mod pages;
pub use pages::*;
pub mod media;
pub use media::*;
