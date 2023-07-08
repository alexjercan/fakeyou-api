// Import the dependencies
use fakeyou_api::{
    tts::{InferenceBody, TtsApi, TtsJobStatus},
    *,
};

fn main() {
    // You can create a default client without any api key.
    // You can also load the API key from environment FAKEYOU_API_KEY.
    // You can also hadcode through `Auth::new(<your_api_key>)`, but it is not recommended.
    let auth = Auth::default();
    let fakeyou = FakeYou::new(auth, FAKEYOU_API_URL);

    // Create the TTS body
    let inference_body = InferenceBody {
        tts_model_token: "TM:ebgxj0j4fvzp".to_string(),
        inference_text: "Hello, World! What should we do today?".to_string(),
        uuid_idempotency_token: None,
    };

    // Call the TTS API
    let inference_result = fakeyou.tts_inference(&inference_body).unwrap();

    // Print the result
    println!("{:?}", inference_result);

    loop {
        // Call the TTS API
        let job_result = fakeyou
            .tts_job(&inference_result.inference_job_token)
            .unwrap();

        // Check if the job is done
        match job_result.state.status {
            TtsJobStatus::Pending => {
                println!("Job is pending");
            }
            TtsJobStatus::Started => {
                println!("Job is started");
            }
            TtsJobStatus::AttemptFailed => {
                println!("Job attempt failed. Trying again...");
            }
            TtsJobStatus::CompleteSuccess => {
                println!("Job completed successfully");
                let output_result = fakeyou
                    .tts_output(&job_result.state.maybe_public_bucket_wav_audio_path.unwrap())
                    .unwrap();

                // Do what you need with the audio file
                std::fs::write("output.wav", output_result.bytes).unwrap();

                break;
            }
            _ => {
                println!("Job failed");

                break;
            }
        }

        // Wait 1 second before trying again
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
