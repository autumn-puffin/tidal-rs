pub mod pkce {
  use base64::prelude::*;
  use rand::{thread_rng, Rng};
  use sha2::{Digest, Sha256};

  #[derive(Debug)]
  pub struct PkceChallenge (String);
  #[derive(Debug)]
  pub struct PkceVerifier (String);

  impl PkceChallenge {
    pub fn as_string(&self) -> String {
      self.0.clone()
    }
  }
  impl PkceVerifier {
    pub fn as_string(&self) -> String {
      self.0.clone()
    }
  }
  
  impl From<PkceChallenge> for String {
    fn from(challenge: PkceChallenge) -> Self {
      challenge.0
    }
  }
  impl From<PkceVerifier> for String {
    fn from(verifier: PkceVerifier) -> Self {
      verifier.0
    }
  }

  pub fn new_random_sha256() -> (PkceChallenge, PkceVerifier) {
    // Generate a random string of 32 chars
    let random: Vec<u8> = (0..32).map(|_| thread_rng().gen::<u8>()).collect();
    let verifier = BASE64_URL_SAFE_NO_PAD.encode(random);

    let digest = Sha256::digest(verifier.as_bytes());
    let challenge = BASE64_URL_SAFE_NO_PAD.encode(digest);

    (PkceChallenge(challenge), PkceVerifier(verifier))
  }

}