use std::convert::TryFrom;
use crate::response::cache::request_builder::{CollectionTtlRequest,
                                              // SendableRequest
};
use crate::{CollectionTtl, IntoBytes, MomentoError, MomentoResult, SimpleCacheClient, utils};
use momento_protos::cache_client::SetUnionRequest;

pub struct SetAddElementsRequest {
    cache_client: SimpleCacheClient,
    cache_name: String,
    set_name: Vec<u8>,
    elements: Vec<Vec<u8>>,
    collection_ttl: Option<CollectionTtl>,
}

impl SetAddElementsRequest {
    pub(crate) fn new(
        cache_client: SimpleCacheClient,
        cache_name: String,
        set_name: Vec<u8>,
        elements: Vec<Vec<u8>>,
    ) -> Self {
        Self {
            cache_client,
            cache_name,
            set_name,
            elements,
            collection_ttl: None,
        }
    }
// }
//
// impl SendableRequest<()> for SetAddElementsRequest {
    pub async fn send(&mut self) -> MomentoResult<()> {
        let collection_ttl = self.collection_ttl.unwrap_or(
            CollectionTtl::default()
        );
        // self.cache_client
        //     .set_union(
        //         &self.cache_name,
        //         self.set_name,
        //         &self.elements,
        //         collection_ttl
        //     )
        //     .await
        // self.cache_client..set_union(cache_name, set_name, elements, policy).await

        let request = prep_request(
            &self.cache_name,
            SetUnionRequest {
                set_name: self.set_name.clone().into_bytes(),
                // elements: convert_vec(&self.elements),
                elements: self.elements.clone(),
                ttl_milliseconds: self.cache_client.expand_ttl_ms(collection_ttl.ttl())?,
                refresh_ttl: collection_ttl.refresh(),
            },
        )?;

        let _ = self.cache_client.data_client.set_union(request).await?.into_inner();
        Ok(())
    }
}


fn prep_request<R>(cache_name: &str, request: R) -> MomentoResult<tonic::Request<R>> {
    utils::is_cache_name_valid(cache_name)?;

    let mut request = tonic::Request::new(request);
    request_meta_data(&mut request, cache_name)?;
    Ok(request)
}

fn request_meta_data<T>(request: &mut tonic::Request<T>, cache_name: &str) -> MomentoResult<()> {
    tonic::metadata::AsciiMetadataValue::try_from(cache_name)
        .map(|value| {
            request.metadata_mut().append("cache", value);
        })
        .map_err(|e| MomentoError::InvalidArgument {
            description: format!("Could not treat cache name as a header value: {e}").into(),
            source: Some(crate::ErrorSource::Unknown(Box::new(e))),
        })
}

// fn convert_vec<E: IntoBytes>(vec: impl IntoIterator<Item = E>) -> Vec<Vec<u8>> {
//     vec.map(|e| e.into_bytes()).collect()
// }


impl CollectionTtlRequest<()> for SetAddElementsRequest {
    fn collection_ttl(&mut self, collection_ttl: CollectionTtl) -> &Self {
        self.collection_ttl = Some(collection_ttl);
        self
    }
}
