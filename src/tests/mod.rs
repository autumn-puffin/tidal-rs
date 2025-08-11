//! Note: Tests require setting up a .env file in the root of the project containing the following variables:
//! - `ClientID`      // A valid Tidal Client ID
//! - `ClientSecret`  // The corresponding Client Secret
//! - `RedirectURI`   // A valid Redirect URI for the Tidal API (only used for User Flow)

mod auth;
mod client;
