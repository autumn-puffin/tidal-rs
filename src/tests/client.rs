use crate::{client::Sessions, utils::client_from_authfile};

#[test]
fn test_sessions() {
  let client = client_from_authfile().unwrap();
  let session_from_auth = client.get_session_from_auth().unwrap();
  let session =client.get_session(&session_from_auth.session_id.to_string()).unwrap();
  assert_eq!(session, session_from_auth);
}