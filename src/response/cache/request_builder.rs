// use crate::{CollectionTtl, MomentoResult};
//
// pub struct CollectionRequestBuilder<TSendFn, TResponse>
// where
//     TSendFn: Fn(CollectionTtl) -> MomentoResult<TResponse>,
// {
//     collection_ttl: Option<CollectionTtl>,
//     pub(crate) send_fn: TSendFn,
// }
//
// impl<TSendFn, TResponse> CollectionRequestBuilder<TSendFn, TResponse>
// where
//     TSendFn: Fn(CollectionTtl) -> MomentoResult<TResponse>,
// {
//     pub(crate) fn new(send_fn: TSendFn) -> Self {
//         Self {
//             collection_ttl: None,
//             send_fn,
//         }
//     }
//     pub fn collection_ttl(&mut self, collection_ttl: CollectionTtl) -> &Self {
//         self.collection_ttl = Some(collection_ttl);
//         self
//     }
// }

use crate::{CollectionTtl,
            // MomentoResult
};

pub trait CollectionTtlRequest<TResponse> { //: SendableRequest<TResponse> {
    fn collection_ttl(&mut self, collection_ttl: CollectionTtl) -> &Self;
}

// pub trait SendableRequest<TResponse> {
//     async fn send(&mut self) -> MomentoResult<TResponse>;
// }
