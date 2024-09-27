use mylib::momento_cache_wrapper::MomentoCacheWrapper;

fn main() {
    println!("Hello, world!");
    let momento_cache_wrapper = MomentoCacheWrapper::new();
    println!("Got momento cache wrapper! closing...");
    momento_cache_wrapper.close();
    println!("Closed momento cache wrapper, exiting...");
}
