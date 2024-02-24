use crate::{MomentoResult};
use crate::cache_client::CacheClient;

pub mod set_add_elements;

pub trait MomentoResponse {}
pub trait MomentoRequest<R: MomentoResponse> {
    async fn send(self: Self, cache_client: &CacheClient) -> MomentoResult<R>;
}
