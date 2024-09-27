#![deny(clippy::all)]

pub mod momento_cache_wrapper;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub struct WeatherItemCacher {
  foo: String
}

#[napi]
impl WeatherItemCacher {
  // #[napi(factory)]
  // pub fn new() -> Self {
  //   WeatherItemCacher {
  //     foo: "bar".to_string()
  //   }
  // }
  
  #[napi(factory)]
  pub fn create() -> Self {
    WeatherItemCacher {
      foo: "bar".to_string()
    }
  }

  #[napi]
  pub fn get_foo(&self) -> String {
    self.foo.clone()
  }
}
// 
// #[test]
// fn test_sum() {
//   println!("TESTING SUM");
//   assert_eq!(sum(1, 2), 3);
// }

// }