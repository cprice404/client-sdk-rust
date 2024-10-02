use std::future::Future;
use std::process::Output;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::Duration;
use momento::CacheClient;
use serde_json::json;
use tokio::task::JoinSet;

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
                    .default_ttl(Duration::from_secs(60))
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

    async fn launch_worker(worker_id: i32, momento_cache_wrapper: Arc<MomentoCacheWrapper>, next_line_counter: Arc<AtomicUsize>, lines: Arc<Vec<String>>) -> i32 {
        let mut my_next_line = next_line_counter.fetch_add(1, Ordering::Relaxed);
        let mut num_lines_processed = 0;
        while my_next_line < lines.len() {
            // println!("Worker {} processing line {}", worker_id, my_next_line);
            // const city = dataPoint.city.name as string;
            // // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access,@typescript-eslint/no-unsafe-assignment
            // const minTemp = dataPoint.main.temp_min as number;
            // // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access,@typescript-eslint/no-unsafe-assignment
            // const maxTemp = dataPoint.main.temp_max as number;
            let line = &lines[my_next_line];
            let json_value: serde_json::Value = serde_json::from_str(line).unwrap_or_else(|_| panic!("Unable to json deserialize line: {}", line));
            let city = &json_value["city"]["name"];
            let min_temp = &json_value["main"]["temp_min"];
            let max_temp = &json_value["main"]["temp_max"];
            let cache_value = json!({
                "minTemp": min_temp,
                "maxTemp": max_temp,
            });
            // println!("Worker {} deserialized weather data from line; city: {}, value: '{}'", worker_id, city, cache_value);
            momento_cache_wrapper.next_cache_client().set(CACHE_NAME, city.to_string(), cache_value.to_string()).await.expect("Error storing value to cache!");
            num_lines_processed += 1;
            my_next_line = next_line_counter.fetch_add(1, Ordering::Relaxed);
        }
        println!("Worker {} got next line {}, which is greater than the total number of lines {}, returning.", worker_id, my_next_line, lines.len());
        num_lines_processed
    }

    pub async fn cache_all_weather_items(self: Arc<Self>, lines: Vec<String>) -> i32 {
        // let self_ref = Arc::new(self);
        // let self_ref = Arc::new(self);
        let next_line_counter = Arc::new(AtomicUsize::new(0));
        // let num_lines = lines.len();
        let lines_ref = Arc::new(lines);
        let mut worker_set = JoinSet::new();
        for i in 1..=1000 {
            let worker_future = MomentoCacheWrapper::launch_worker(i, self.clone(), next_line_counter.clone(), lines_ref.clone()); 
            worker_set.spawn(worker_future);
        }
        let results = worker_set.join_all().await;

        println!("Workers complete; num items processed: {:?}", results);

        results.into_iter().reduce(|a, b| a + b ).unwrap_or(0)
    }

    pub async fn set(&self, key: &str, value: &str) -> bool{
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
