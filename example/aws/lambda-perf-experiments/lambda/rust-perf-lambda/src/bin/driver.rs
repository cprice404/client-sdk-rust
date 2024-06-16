use std::io;
use thiserror::Error;
use rust_loadgen_lambda::loadgen::{LoadGenError, run_loadgen};

#[derive(Error, Debug)]
pub enum Error {
    #[error("unknown error")]
    Unknown(#[from] io::Error),
    #[error("loadgen error")]
    LoadGenError(#[from] LoadGenError)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");
    run_loadgen().await?;
    Ok(())
}
