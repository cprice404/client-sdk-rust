use momento::response::Get;
use momento::{CacheClient, CollectionTtl, CredentialProvider, MomentoError, SimpleCacheClientBuilder};
use std::process;
use std::time::Duration;
use momento::config::grpc_configuration::GrpcConfiguration;
use momento::config::transport_strategy::TransportStrategy;
use momento::requests::cache::sorted_set::sorted_set_put_elements::SortedSetPutElementsRequest;

#[tokio::main]
async fn main() -> Result<(), MomentoError> {
    // Initializing Momento
    let mut cache_client = match SimpleCacheClientBuilder::new(
        CredentialProvider::from_env_var("MOMENTO_AUTH_TOKEN".to_string())?,
        Duration::from_secs(60),
    ) {
        Ok(client) => client,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    }
    .build();

    // Creating a cache named "cache"
    let cache_name = String::from("cache");
    match cache_client.create_cache(&cache_name).await {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{err}");
        }
    }

    // List the caches
    println!("Listing caches:");
    match cache_client.list_caches().await {
        Ok(list_cache_result) => {
            for listed_cache in list_cache_result.caches {
                println!("{}", listed_cache.cache_name);
            }
        }
        Err(err) => {
            eprintln!("{err}");
        }
    };
    println!();

    // Sets key with default TTL and get value with that key
    let key = String::from("my_key");
    let value = String::from("my_value");
    println!("Setting key: {key}, value: {value}");
    match cache_client
        .set(&cache_name, key.clone(), value.clone(), None)
        .await
    {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{err}");
        }
    };
    match cache_client.get(&cache_name, key.clone()).await {
        Ok(r) => match r {
            Get::Hit { value } => {
                let v: String = value.try_into().expect("I stored a string!");
                println!("Got value: {v}");
            }
            Get::Miss => {
                println!("Cache miss!");
            }
        },
        Err(err) => {
            eprintln!("{err}");
        }
    };
    // Permanently deletes cache
    match cache_client.delete_cache(&cache_name).await {
        Ok(_) => {
            println!("Permanently deleted cache named, {cache_name}");
        }
        Err(err) => {
            eprintln!("{err}");
        }
    };
    
    new_stuff().await?;
    new_new_stuff().await?;
    
    Ok(())
}

async fn new_stuff() -> Result<(), MomentoError> {
    // Configuration builders | phased builders for these are going to be the easiest ways to steer
    //                        | users to providing all the required fields, while still allowing
    //                        | optional arguments now and in the future. The 'build' may not strictly
    //                        | be necessary if these are pretty much just structs we're creating.
    let config = momento::config::configuration::Configuration::builder(
        TransportStrategy::builder(
            GrpcConfiguration::builder(
                Duration::from_secs(60)
            ).build()
        ).build()
    ).build();

    // Credential Provider builders | this one needs a builder because it will parse the tokens and
    //                              | stuff when you call 'build'. we can have some factory fns that
    //                              | skip the exposure to the builder in the common case, or force
    //                              | people to see the builder for consistency
    let cred_provider = CredentialProvider::builder()
        .from_env_var("MOMENTO_API_KEY".to_string())
        .base_endpoint("foo.com")
        .build()?;

    // Cache Client builders | this one will need a builder, because the 'build' function gates the
    //                       | establishment of connections etc. can probably be similar to the config builder
    let cache_client = CacheClient::new(cred_provider, config, Duration::from_secs(60))?;

    // Request builders | these won't really need to have a builder because they don't need a 'build' function,
    //                  | because there are no resources that need to be initialized on 'build'. but we could
    //                  | make builders anyway just for consistency
    let sorted_set_put_elements_request = SortedSetPutElementsRequest::new(
        "cache".to_string(),
        "key".to_string(),
        vec![]
    ).with_ttl(CollectionTtl::of(Duration::from_secs(60)));
    
    Ok(())
}


async fn new_new_stuff() -> Result<(), MomentoError> {
    // Configuration builders | phased builders for these are going to be the easiest ways to steer
    //                        | users to providing all the required fields, while still allowing
    //                        | optional arguments now and in the future. The 'build' may not strictly
    //                        | be necessary if these are pretty much just structs we're creating, but
    //                        | if we want to do phased builders with potentially multiple exit paths
    //                        | then we will need thm.
    let config = momento::config::configuration::Configuration::builder()
        .transport_strategy(
            TransportStrategy::builder()
                .grpc_configuration(
                    GrpcConfiguration::builder()
                        .deadline(Duration::from_secs(60))
                        .build()
                ).build()
        ).build();

    // Credential Provider builders | this one needs a builder because it will parse the tokens and
    //                              | stuff when you call 'build'. we can have some factory fns that
    //                              | skip the exposure to the builder in the common case, or force
    //                              | people to see the builder for consistency
    let cred_provider = CredentialProvider::builder()
        .from_env_var("MOMENTO_API_KEY".to_string())
        .base_endpoint("foo.com")
        .build()?;

    // Cache Client builders | this one will need a builder, because the 'build' function gates the
    //                       | establishment of connections etc. can probably be similar to the config builder
    let cache_client = CacheClient::builder()
        .credential_provider(cred_provider)
        .config(config)
        .duration(Duration::from_secs(60))
        .build()?;

    // Request builders | these won't really need to have a builder because they don't need a 'build' function,
    //                  | because there are no resources that need to be initialized on 'build'. but we could
    //                  | make builders anyway just for consistency
    let sorted_set_put_elements_request = SortedSetPutElementsRequest::new(
        "cache".to_string(),
        "key".to_string(),
        vec![]
    ).with_ttl(CollectionTtl::of(Duration::from_secs(60)));

    Ok(())
}
