use std::fs::read_to_string;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let momento_wrapper = momento_wrapper::MomentoCacheWrapper::new();
    println!("Instantiated momento wrapper");
    momento_wrapper.set("foo", "BAR").await;
    println!("Stored a value in the cache!");
    
    // let lines: Vec<String> = vec![
    //     r#"{"city":{"id":14256,"name":"Azadshahr","findname":"AZADSHAHR","country":"IR","coord":{"lon":48.570728,"lat":34.790878},"zoom":10},"time":1554462304,"main":{"temp":287.07,"pressure":1022,"humidity":71,"temp_min":284.15,"temp_max":289.15},"wind":{"speed":4.1,"deg":340},"clouds":{"all":90},"weather":[{"id":804,"main":"Clouds","description":"overcast clouds","icon":"04d"}]}"#.to_string(),
    //     r#"{"city":{"id":14256,"name":"Azadshahr","findname":"AZADSHAHR","country":"IR","coord":{"lon":48.570728,"lat":34.790878},"zoom":10},"time":1554462304,"main":{"temp":287.07,"pressure":1022,"humidity":71,"temp_min":284.15,"temp_max":289.15},"wind":{"speed":4.1,"deg":340},"clouds":{"all":90},"weather":[{"id":804,"main":"Clouds","description":"overcast clouds","icon":"04d"}]}"#.to_string(),
    //     r#"{"city":{"id":56166,"name":"Jilib","findname":"JILIB","country":"SO","coord":{"lon":42.785351,"lat":0.48829},"zoom":9},"time":1554462304,"main":{"temp":309.929,"pressure":1008.39,"humidity":36,"temp_min":309.929,"temp_max":309.929},"wind":{"speed":5.72,"deg":104},"clouds":{"all":0},"weather":[{"id":800,"main":"Clear","description":"clear sky","icon":"01d"}]}"#.to_string(),
    //     r#"{"city":{"id":55671,"name":"Kismaayo","findname":"KISMAAYO","country":"SO","coord":{"lon":42.545361,"lat":-0.35817},"zoom":6},"time":1554462304,"main":{"temp":301.329,"pressure":1009.87,"humidity":100,"temp_min":301.329,"temp_max":301.329},"wind":{"speed":6.12,"deg":103.5},"clouds":{"all":0},"weather":[{"id":800,"main":"Clear","description":"clear sky","icon":"01d"}]}"#.to_string(),
    //     r#"{"city":{"id":60019,"name":"Eyl","findname":"EYL","country":"SO","coord":{"lon":49.816399,"lat":7.9803},"zoom":5},"time":1554462304,"main":{"temp":302.229,"pressure":1010.67,"humidity":80,"temp_min":302.229,"temp_max":302.229},"wind":{"speed":5.02,"deg":111.5},"clouds":{"all":0},"weather":[{"id":800,"main":"Clear","description":"clear sky","icon":"01d"}]}"#.to_string(),
    //     r#"{"city":{"id":23814,"name":"Kahriz","findname":"KAHRIZ","country":"IR","coord":{"lon":47.055302,"lat":34.383801},"zoom":7},"time":1554462304,"main":{"temp":285.62,"pressure":1021,"humidity":66,"temp_min":285.15,"temp_max":286.15},"wind":{"speed":2.1,"deg":150},"clouds":{"all":75},"weather":[{"id":803,"main":"Clouds","description":"broken clouds","icon":"04d"}]}"#.to_string(),
    //     r#"{"city":{"id":53654,"name":"Mogadishu","findname":"MOGADISHU","country":"SO","coord":{"lon":45.34375,"lat":2.03711},"zoom":1},"time":1554462304,"main":{"temp":305.15,"pressure":1011,"humidity":59,"temp_min":305.15,"temp_max":305.15},"wind":{"speed":5.1,"deg":100},"clouds":{"all":75},"weather":[{"id":803,"main":"Clouds","description":"broken clouds","icon":"04d"}]}"#.to_string(),
    //     r#"{"city":{"id":62691,"name":"Ceerigaabo","findname":"CEERIGAABO","country":"SO","coord":{"lon":47.36795,"lat":10.61616},"zoom":7},"time":1554462304,"main":{"temp":303.229,"pressure":1008.79,"humidity":36,"temp_min":303.229,"temp_max":303.229},"wind":{"speed":0.87,"deg":1.5},"clouds":{"all":0},"weather":[{"id":800,"main":"Clear","description":"clear sky","icon":"01d"}]}"#.to_string(),
    //     r#"{"city":{"id":64460,"name":"Beledweyne","findname":"BELEDWEYNE","country":"SO","coord":{"lon":45.203609,"lat":4.73583},"zoom":7},"time":1554462304,"main":{"temp":310.029,"pressure":1007.83,"humidity":31,"temp_min":310.029,"temp_max":310.029},"wind":{"speed":3.82,"deg":103},"clouds":{"all":0},"weather":[{"id":800,"main":"Clear","description":"clear sky","icon":"01d"}]}"#.to_string(),
    //     r#"{"city":{"id":64536,"name":"Baydhabo","findname":"BAYDHABO","country":"SO","coord":{"lon":43.649799,"lat":3.11383},"zoom":7},"time":1554462304,"main":{"temp":309.079,"pressure":1007.83,"humidity":35,"temp_min":309.079,"temp_max":309.079},"wind":{"speed":4.82,"deg":105},"clouds":{"all":0},"weather":[{"id":800,"main":"Clear","description":"clear sky","icon":"01d"}]}"#.to_string(),
    // ]; 


    // fn read_lines(filename: &str) -> Vec<String> {
    let lines = read_to_string("../../../../../../scratch/weather_16.json")
            .unwrap()  // panic on possible file-reading errors
            .lines()  // split the string into an iterator of string slices
            .map(String::from)  // make each slice into a string
            .collect();  // gather them together into a vector
    // }
    
    let momento_wrapper_arc = Arc::new(momento_wrapper);
    
    let num_cached_items = momento_wrapper_arc.clone().cache_all_weather_items(lines).await;
    println!("Cached {} items", num_cached_items);
    
    momento_wrapper_arc.close().await;
    println!("Closed momento wrapper");
}
