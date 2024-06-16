use std::sync::Arc;
use std::time::Duration;
use momento::cache::{configurations};
use momento::{CacheClient, CredentialProvider, MomentoError};
use thiserror::Error;
use tokio::task::JoinSet;

#[derive(Error, Debug)]
pub enum LoadGenError {
    #[error("MomentoError: {0}")]
    MomentoError(#[from] MomentoError),
    #[error("JoinError: {0}")]
    JoinError(#[from] tokio::task::JoinError),
}

const CACHE_NAME: &str = "cache";

pub async fn run_loadgen() -> Result<(), LoadGenError> {
    println!("Hello from loadgen!");
    let cache_client = CacheClient::builder()
        .default_ttl(Duration::from_secs(60))
        .configuration(configurations::Laptop::latest())
        .credential_provider(CredentialProvider::from_env_var("MOMENTO_API_KEY")?)
        .build()?;
    let cache_client_arc = Arc::new(cache_client);

    let num_workers = 20;
    let mut workers_set = JoinSet::new();
    for i in 0..num_workers {
        workers_set.spawn(run_worker(cache_client_arc.clone(), i));
    }
    while let Some(worker_result) = workers_set.join_next().await {
        println!("Worker finished with result: {:?}", worker_result?);
    }

    Ok(())
}

async fn run_worker(cache_client: Arc<CacheClient>, worker_id: usize) -> Result<(), LoadGenError> {
    let cache_key_string = format!("my-cache-key-{}", worker_id);
    let cache_key = cache_key_string.as_str();
    cache_client.set(CACHE_NAME, cache_key, format!("my-cache-value-{}", worker_id)).await?;
    let get_result: String = cache_client.get(CACHE_NAME, cache_key).await?.try_into()?;
    println!("Worker {} successfully retrieved cache value for key {}: {}",
             worker_id,
             cache_key, get_result);

    Ok(())
} 
