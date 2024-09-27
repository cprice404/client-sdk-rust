#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let momento_wrapper = momento_wrapper::MomentoCacheWrapper::new();
    println!("Instantiated momento wrapper: {:?}", momento_wrapper);
    momento_wrapper.set("foo", "BAR").await;
    println!("Stored a value in the cache!");
    momento_wrapper.close();
    println!("Closed momento wrapper");
}
