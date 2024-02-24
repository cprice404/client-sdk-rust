use momento_protos::cache_client::SetUnionRequest;
use crate::{CollectionTtl, IntoBytes, MomentoResult, SimpleCacheClient};
use crate::requests::cache::{MomentoRequest, MomentoResponse};
use crate::simple_cache_client::prep_request;

pub struct SetAddElementsRequest<S: IntoBytes, E: IntoBytes> {
    cache_name: String,
    set_name: S,
    elements: Vec<E>,
    collection_ttl: Option<CollectionTtl>
}

impl <S: IntoBytes, E: IntoBytes> MomentoRequest<SetAddElements> for SetAddElementsRequest<S, E> {
    async fn send(self: Self, cache_client: &SimpleCacheClient) -> MomentoResult<SetAddElements> {
        let collection_ttl = self.collection_ttl.unwrap_or_default();
        let elements = self.elements.into_iter().map(|e| e.into_bytes()).collect();
        let request = prep_request(
            &self.cache_name,
            SetUnionRequest {
                set_name: self.set_name.into_bytes(),
                elements,
                ttl_milliseconds: cache_client.expand_ttl_ms(collection_ttl.ttl())?,
                refresh_ttl: collection_ttl.refresh(),
            },
        )?;

        let _ = cache_client.data_client.clone().set_union(request).await?.into_inner();
        Ok(SetAddElements::Success {})
    }
}

pub enum SetAddElements {
    Success {},
}

impl MomentoResponse for SetAddElements {}
