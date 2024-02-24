use crate::{MomentoResult, SimpleCacheClient};

pub mod set_add_elements;

trait MomentoResponse {}
trait MomentoRequest<R: MomentoResponse> {
    async fn send(self: Self, cache_client: &SimpleCacheClient) -> MomentoResult<R>;
}
