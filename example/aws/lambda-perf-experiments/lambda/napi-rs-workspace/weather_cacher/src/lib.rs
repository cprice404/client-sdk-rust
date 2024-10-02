#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::sync::Arc;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub struct WeatherItemCacher {
  momento_wrapper: Arc<momento_wrapper::MomentoCacheWrapper>
}

#[napi]
impl WeatherItemCacher {
  #[napi(factory)]
  pub fn create() -> Self {
    WeatherItemCacher {
      momento_wrapper: Arc::new(momento_wrapper::MomentoCacheWrapper::new())
    }
  }
  
  #[napi]
  pub async fn cache_all_weather_items(&self, lines: Vec<String>) -> i32 {
    self.momento_wrapper.clone().cache_all_weather_items(lines).await
  }
  
  #[napi]
  pub async fn set(&self, key: String, value: String) -> bool {
    self.momento_wrapper.set(&key, &value).await
  }
  
  #[napi]
  pub async fn close(&self) {
    self.momento_wrapper.close().await;
  }
}