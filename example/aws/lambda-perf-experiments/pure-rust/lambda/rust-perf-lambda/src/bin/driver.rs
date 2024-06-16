use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unknown error")]
    Unknown(#[from] io::Error)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");
    Ok(())
}
