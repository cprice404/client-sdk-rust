use momento::response::cache_get_response::MomentoGetStatus;
use momento::simple_cache_client::{SimpleCacheClient, SimpleCacheClientBuilder};
use std::{env, thread, time};
use std::num::NonZeroU64;
use std::process;
use std::str;
use std::str::Utf8Error;

async fn do_get_set(cache_client: &mut SimpleCacheClient) {
    let cache_name = "tacocache";

    // Sets key with default TTL and get value with that key
    let key = String::from("my_key");
    let value = String::from("my_value");

    match cache_client
        .set(&cache_name, key.clone(), value.clone(), None)
        .await
    {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err);
        }
    };

    match cache_client.get(&cache_name, key.clone()).await {
        Ok(r) => match r.result {
            MomentoGetStatus::HIT => match str::from_utf8(&r.value) {
                Ok(s) => println!("cache hit! result: {}", s),
                Err(_) => println!("Error converting result to utf-8 string!")
            }
            MomentoGetStatus::MISS => println!("cache miss"),
            _ => println!("error occurred"),
        },
        Err(err) => {
            eprintln!("{}", err);
        }
    };

}

#[tokio::main]
async fn main() {
    // Initializing Momento
    let auth_token =
        env::var("MOMENTO_AUTH_TOKEN").expect("env var MOMENTO_AUTH_TOKEN must be set");
    let item_default_ttl_seconds = 60;
    let mut cache_client = match SimpleCacheClientBuilder::new(
        auth_token,
        NonZeroU64::new(item_default_ttl_seconds).expect("expected a non-zero number"),
    ) {
        Ok(client) => client,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
    .build();

    do_get_set(&mut cache_client).await;
    println!("Sleeping for 6 minutes to try to trigger NLB idle timeout");
    thread::sleep(time::Duration::from_secs(6 * 60));
    do_get_set(&mut cache_client).await;
}
