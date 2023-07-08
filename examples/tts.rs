// Import the dependencies
use fakeyou_api::{tts::InferenceBody, util::tts::TtsApiSync, *};

fn main() {
    // You can create a default client without any api key.
    // You can also load the API key from environment FAKEYOU_API_KEY.
    // You can also hadcode through `Auth::new(<your_api_key>)`, but it is not recommended.
    let auth = Auth::default();
    let fakeyou = FakeYou::new(auth, FAKEYOU_API_URL);

    // Create the TTS body
    let inference_body =
        InferenceBody::new("TM:ebgxj0j4fvzp", "Hello, World! What should we do today?");

    // Call the TTS API
    // This uses the util module of this crate and will block the thread until the task is done
    let output_result = fakeyou.create_tts_task(&inference_body).unwrap();

    // Do what you need with the audio file
    std::fs::write("output.wav", output_result.bytes).unwrap();
}
