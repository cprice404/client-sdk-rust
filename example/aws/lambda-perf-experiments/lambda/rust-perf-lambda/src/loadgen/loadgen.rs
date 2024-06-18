use std::sync::Arc;
use std::time::Duration;
use hdrhistogram::{Histogram, SyncHistogram};
use hdrhistogram::sync::Recorder;
// use histogram::{AtomicHistogram};
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
    #[error("HistogramCreationError: {0}")]
    HistogramCreationError(#[from] hdrhistogram::CreationError),
    #[error("HistogramRecordError: {0}")]
    HistogramRecordError(#[from] hdrhistogram::RecordError)
}

const CACHE_NAME: &str = "cache";

pub async fn run_loadgen() -> Result<(), LoadGenError> {
    let start_time = std::time::Instant::now();
    println!("Hello from loadgen!");
    let cache_client = CacheClient::builder()
        .default_ttl(Duration::from_secs(60))
        .configuration(configurations::Laptop::latest())
        .credential_provider(CredentialProvider::from_env_var("MOMENTO_API_KEY")?)
        .build()?;
    cache_client.get(CACHE_NAME, "my-cache-key").await?;
    let cache_client_arc = Arc::new(cache_client);

    // let set_histogram = Arc::new(AtomicHistogram::new(4, 7)?);
    let mut set_histogram: SyncHistogram<u32> = Histogram::<u32>::new(4)?.into();
    let mut get_histogram: SyncHistogram<u32> = Histogram::<u32>::new(4)?.into();

    let num_workers = 5;
    // let num_workers = 1;
    let run_time = Duration::from_secs(30);
    let mut workers_set = JoinSet::new();
    for i in 0..num_workers {
        workers_set.spawn(run_worker(
            cache_client_arc.clone(),
            set_histogram.recorder(),
            get_histogram.recorder(),
            i,
            run_time));
    }
    while let Some(worker_result) = workers_set.join_next().await {
        println!("Worker finished with result: {:?}", worker_result?);
    }
    
    set_histogram.refresh();
    get_histogram.refresh();
    
    let p50 = set_histogram.value_at_percentile(50.0);
    println!("Set latency p50: {} ms", p50);
    let p50 = get_histogram.value_at_percentile(50.0);
    println!("Get latency p50: {} ms", p50);
    
    let total_request_count = set_histogram.len() + get_histogram.len();
    println!("Total request count: {}", total_request_count);
    let tps = total_request_count as f64 / start_time.elapsed().as_secs() as f64;
    println!("TPS: {}", tps);
    
    print_histogram_summary(&set_histogram, "Set");
    print_histogram_summary(&get_histogram, "Get");

    // let set_histogram_snapshot = set_histogram.drain();
    // set_histogram_snapshot.

    Ok(())
}

async fn run_worker(
    cache_client: Arc<CacheClient>,
    mut set_histogram_recorder: Recorder<u32>,
    mut get_histogram_recorder: Recorder<u32>,
    worker_id: usize,
    run_time: Duration
) -> Result<u32, LoadGenError> {
    let cache_key_string = format!("my-cache-key-{}", worker_id);
    let cache_key = cache_key_string.as_str();
    
    let start_time = std::time::Instant::now();
    let mut i = 0;
    while start_time.elapsed() < run_time {
        i += 1;
        let set_start_time = std::time::Instant::now();
        cache_client.set(CACHE_NAME, cache_key, format!("my-cache-value-{}-{}", worker_id, i)).await?;
        let set_elapsed = set_start_time.elapsed().as_millis();
        // println!("Set latency: {} ms", set_elapsed);
        // set_histogram.increment(elapsed as u64)?;
        set_histogram_recorder.record(set_elapsed as u64)?;

        let get_start_time = std::time::Instant::now();
        cache_client.get(CACHE_NAME, cache_key).await?;
        let get_elapsed = get_start_time.elapsed().as_millis();
        get_histogram_recorder.record(get_elapsed as u64)?;
        // println!("Worker {} successfully retrieved cache value for key {}: {}",
        //          worker_id,
        //          cache_key, get_result);
    }

    Ok(i)
}

fn print_histogram_summary(histogram: &Histogram<u32>, name: &str) {
    println!("cumulative {} latencies:", name);
    println!("  count: {}", histogram.len());
    println!("    min: {} ms", histogram.min());
    println!("    p50: {} ms", histogram.value_at_percentile(50.0));
    println!("    p90: {} ms", histogram.value_at_percentile(50.0));
    println!("    p99: {} ms", histogram.value_at_percentile(50.0));
    println!("  p99.9: {} ms", histogram.value_at_percentile(50.0));
    println!("    max: {} ms", histogram.max());
}
