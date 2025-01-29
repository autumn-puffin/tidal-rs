//! Interfaces for clients interacting with the Tidal API

pub mod auth;
pub use auth::{Auth, ClientFlow, Credentials, DeviceFlow, RefreshFlow, Sessions, UserFlow};
pub mod catalogue;
pub use catalogue::{AlbumCatalogue, ArtistCatalogue, Catalogue, PlaylistCatalogue, TrackCatalogue, VideoCatalogue};
pub mod users;
pub use users::Users;
