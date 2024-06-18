use std::time::Duration;
use serde_json::Value;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use momento::cache::configurations::Lambda;
use momento::{CacheClient, CredentialProvider};
use rust_loadgen_lambda::loadgen::run_loadgen;

const DEFAULT_TTL: Duration = Duration::from_secs(60);

lazy_static::lazy_static! {
    static ref CACHE_CLIENT: CacheClient = CacheClient::builder()
    .default_ttl(DEFAULT_TTL)
    .configuration(Lambda::latest())
    .credential_provider(CredentialProvider::from_env_var("MOMENTO_API_KEY")
        .expect("Unable to construct Momento CredentialProvider using env var MOMENTO_API_KEY"))
    .build()
    .expect("Unable to construct Momento CacheClient");
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(_event: LambdaEvent<Value>) -> Result<(), Error> {
    
    run_loadgen().await?;
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
