use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use momento::cache::SetResponse;

#[derive(Debug)]
pub struct MomentoCacheWrapper {
    pub cache_clients: Vec<momento::CacheClient>,
    num_clients: u64,
    next_client_index: AtomicU64
}

impl Default for MomentoCacheWrapper {
    fn default() -> Self {
        Self::new()
    }
}

const CACHE_NAME: &str = "cache";

impl MomentoCacheWrapper {
    pub fn new() -> Self {
        let next_client_index = AtomicU64::new(0);
        let num_clients = 10;
        let clients: Vec<momento::CacheClient> = (0..num_clients)
            .map(|_| {
                momento::CacheClient::builder()
                    .default_ttl(Duration::from_secs(60 * 20))
                    .configuration(momento::cache::configurations::Laptop::latest())
                    .credential_provider(
                        momento::CredentialProvider::from_env_var("MOMENTO_API_KEY".to_string())
                            .expect("API key should be valid"),
                    ).build()
                    .expect("Error when instantiating cache client")
            })
            .collect();
        MomentoCacheWrapper {
            cache_clients: clients,
            num_clients,
            next_client_index
        }
    }

    pub async fn set(&self, key: &str, value: &str) -> bool{
        println!("Cache key: {:?} with value: {:?}", key, value);
        self.next_cache_client().set(CACHE_NAME, key, value).await.expect("Error when setting value");
        true
    }

    fn next_cache_client(&self) -> &momento::CacheClient {
        let client_index: usize = (self.next_client_index.fetch_add(1, Ordering::Relaxed) % self.num_clients) as usize;
        // println!("Using client #{:?}", client_index.clone());
        &self.cache_clients[client_index]
    }

    pub async fn close(&self) {
        // nothing to do here
    }
}
