// Import the dependencies
use fakeyou_api::{voices::*, *};

fn main() {
    // You can create a default client without any api key.
    // You can also load the API key from environment FAKEYOU_API_KEY.
    // You can also hadcode through `Auth::new(<your_api_key>)`, but it is not recommended.
    let auth = Auth::default();
    let fakeyou = FakeYou::new(auth, FAKEYOU_API_URL);

    // Call the TTS API
    // This uses the util module of this crate and will block the thread until the task is done
    let voices_result = fakeyou.tts_list().unwrap();

    // Do what you need with the result
    println!("{:#?}", voices_result);
}
