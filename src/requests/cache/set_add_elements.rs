use momento_protos::cache_client::SetUnionRequest;
use crate::{CollectionTtl, IntoBytes, MomentoResult, SimpleCacheClient};
use crate::requests::cache::MomentoRequest;

pub struct SetAddElementsRequest<S: IntoBytes, E: IntoBytes> {
    set_name: S,
    elements: E,
    collection_ttl: Option<CollectionTtl>
}

impl <S, E> MomentoRequest<SetAddElements> for SetAddElementsRequest<S, E> {
    async fn send(cache_client: &SimpleCacheClient) -> MomentoResult<SetAddElements> {
        let request = self.prep_request(
            cache_name,
            SetUnionRequest {
                set_name: set_name.into_bytes(),
                elements: crate::simple_cache_client::convert_vec(elements),
                ttl_milliseconds: self.expand_ttl_ms(policy.ttl())?,
                refresh_ttl: policy.refresh(),
            },
        )?;

        let _ = self.data_client.set_union(request).await?.into_inner();
        Ok(())
    }
}

pub enum SetAddElements {
    Success {},
}
