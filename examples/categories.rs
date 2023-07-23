// Import the dependencies
use fakeyou_api::{category::*, *};

fn main() {
    // You can create a default client without any api key.
    // You can also load the API key from environment FAKEYOU_API_KEY.
    // You can also hadcode through `Auth::new(<your_api_key>)`, but it is not recommended.
    let auth = Auth::default();
    let fakeyou = FakeYou::new(auth, FAKEYOU_API_URL);

    // Call the Categories API
    let categories_result = fakeyou.tts_categories().unwrap();

    // Do what you need with the result
    println!("{:#?}", categories_result);
}
