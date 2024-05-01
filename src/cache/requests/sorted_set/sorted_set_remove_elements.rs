use momento_protos::cache_client::sorted_set_remove_request::{RemoveElements, Some};
use momento_protos::cache_client::SortedSetRemoveRequest;

use crate::cache::requests::MomentoRequest;
use crate::utils::prep_request_with_timeout;
use crate::{CacheClient, IntoBytes, MomentoResult};

/// Remove multiple elements from the sorted set.
///
/// # Arguments
/// * `cache_name` - The name of the cache containing the sorted set.
/// * `sorted_set_name` - The name of the sorted set to remove elements from.
/// * `values` - The values to remove. Must be able to be converted to a `Vec<u8>`.
///
/// # Examples
/// Assumes that a CacheClient named `cache_client` has been created and is available.
/// ```
/// # fn main() -> anyhow::Result<()> {
/// # use momento_test_util::create_doctest_cache_client;
/// # tokio_test::block_on(async {
/// use momento::cache::SortedSetRemoveElementsRequest;
/// # let (cache_client, cache_name) = create_doctest_cache_client();
/// let sorted_set_name = "sorted_set";
///
/// let remove_elements_request = SortedSetRemoveElementsRequest::new(
///     cache_name,
///     sorted_set_name,
///     vec!["value1", "value2"]
/// );
///
/// match cache_client.send_request(remove_elements_request).await {
///     Ok(_) => println!("Elements removed from sorted set"),
///     Err(e) => eprintln!("Error removing elements from sorted set: {}", e),
/// }
/// # Ok(())
/// # })
/// # }
/// ```
pub struct SortedSetRemoveElementsRequest<S: IntoBytes, V: IntoBytes> {
    cache_name: String,
    sorted_set_name: S,
    values: Vec<V>,
}

impl<S: IntoBytes, V: IntoBytes> SortedSetRemoveElementsRequest<S, V> {
    pub fn new(cache_name: impl Into<String>, sorted_set_name: S, values: Vec<V>) -> Self {
        Self {
            cache_name: cache_name.into(),
            sorted_set_name,
            values,
        }
    }
}

impl<S: IntoBytes, V: IntoBytes> MomentoRequest for SortedSetRemoveElementsRequest<S, V> {
    type Response = SortedSetRemoveElements;

    async fn send(self, cache_client: &CacheClient) -> MomentoResult<SortedSetRemoveElements> {
        let values = self.values.into_iter().map(|v| v.into_bytes()).collect();
        let set_name = self.sorted_set_name.into_bytes();
        let cache_name = &self.cache_name;
        let request = prep_request_with_timeout(
            cache_name,
            cache_client.configuration.deadline_millis(),
            SortedSetRemoveRequest {
                set_name,
                remove_elements: Some(RemoveElements::Some(Some { values })),
            },
        )?;

        let _ = cache_client
            .data_client
            .clone()
            .sorted_set_remove(request)
            .await?;
        Ok(SortedSetRemoveElements {})
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SortedSetRemoveElements {}
