use crate::{
  api::{PagingResponse, User, UserClient, UserSubscription},
  auth::Auth,
  Result,
};

/// Trait for user related api functions
pub trait Users: Auth {
  /// Gets a user by their user_id
  fn get_user(&self, user_id: &u64) -> Result<User>;
  /// Get a user's subscription
  fn get_user_subscription(&self, user_id: &u64) -> Result<UserSubscription>;
  /// Get a user's clients
  fn get_user_clients(&self, user_id: &u64) -> Result<PagingResponse<UserClient>>;

  /// Get the current user
  fn get_current_user(&self) -> Result<User> {
    let credentials = self.get_credentials()?;
    self.get_user(credentials.user_id().ok_or(UsersError::NoCurrentUser)?)
  }
  /// Get the current user's subscription
  fn get_current_user_subscription(&self) -> Result<UserSubscription> {
    let credentials = self.get_credentials()?;
    self.get_user_subscription(credentials.user_id().ok_or(UsersError::NoCurrentUser)?)
  }
  /// Get the current user's clients
  fn get_current_user_clients(&self) -> Result<PagingResponse<UserClient>> {
    let credentials = self.get_credentials()?;
    self.get_user_clients(credentials.user_id().ok_or(UsersError::NoCurrentUser)?)
  }
}

#[derive(Debug)]
pub enum UsersError {
  NoCurrentUser,
}
