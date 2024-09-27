use std::time::Duration;
use momento;

pub struct MomentoCacheWrapper {
  pub cache_client: momento::CacheClient
}

impl Default for MomentoCacheWrapper {
    fn default() -> Self {
        Self::new()
    }
}

impl MomentoCacheWrapper {
  pub fn new() -> Self {
      let cache_client = momento::CacheClient::builder()
          .default_ttl(Duration::from_secs(60))
          .configuration(momento::cache::configurations::Laptop::latest())
          .credential_provider(
              momento::CredentialProvider::from_env_var("MOMENTO_API_KEY".to_string())
                  .expect("API key should be valid"),
          )
          .build()
          .expect("Error when instantiating cache client");
    MomentoCacheWrapper {
      cache_client
    }
  }

  pub fn close(&self) {
      // nothing to do here
  }
}
