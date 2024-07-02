use serde_json::Value;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use rust_loadgen_lambda::loadgen::run_loadgen;

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
