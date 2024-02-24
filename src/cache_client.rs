use std::convert::TryInto;
use std::time::Duration;
use momento_protos::cache_client::scs_client::ScsClient;
use tonic::codegen::InterceptedService;
use tonic::transport::Channel;
use crate::grpc::header_interceptor::HeaderInterceptor;
use crate::{IntoBytes, MomentoResult, utils};
use crate::requests::cache::MomentoRequest;
use crate::requests::cache::set_add_elements::{SetAddElements, SetAddElementsRequest};

pub struct CacheClient {
    pub(crate) data_client: ScsClient<InterceptedService<Channel, HeaderInterceptor>>,
    item_default_ttl: Duration,
}

impl CacheClient {
    /* public API */
    pub async fn set_add_elements<E: IntoBytes>(self: &Self, cache_name: String, set_name: impl IntoBytes, elements: Vec<E>) -> MomentoResult<SetAddElements> {
        let request = SetAddElementsRequest::new(cache_name, set_name, elements);
        MomentoRequest::send(request, self).await
    }


    /* helper fns */
    pub(crate) fn expand_ttl_ms(&self, ttl: Option<Duration>) -> MomentoResult<u64> {
        let ttl = ttl.unwrap_or(self.item_default_ttl);
        utils::is_ttl_valid(ttl)?;

        Ok(ttl.as_millis().try_into().unwrap_or(i64::MAX as u64))
    }
}
