//! Tidal API model
//!
//! This module contains the models for data returned from the Tidal API, including catalogue media,
//! page data, various enums, and other data structures.

pub mod enums;
pub mod media;
pub mod pages;
pub mod paging;
pub mod playback;
pub mod user;
pub use enums::*;
pub use media::*;
pub use pages::*;
pub use paging::*;
pub use playback::*;
pub use user::*;
