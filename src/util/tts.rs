use crate::tts::*;
use crate::*;

pub trait TtsApiSync {
    fn create_tts_task(&self, inference_body: &InferenceBody) -> ApiResult<TtsOutputResult>;
}

impl TtsApiSync for FakeYou {
    fn create_tts_task(&self, inference_body: &InferenceBody) -> ApiResult<TtsOutputResult> {
        let inference_result = self.tts_inference(inference_body)?;

        loop {
            let job_result = self.tts_job(&inference_result.inference_job_token);

            let status = match job_result {
                Ok(job_result) => match job_result.state.status {
                    TtsJobStatus::Pending => None,
                    TtsJobStatus::Started => None,
                    TtsJobStatus::AttemptFailed => None,
                    TtsJobStatus::CompleteSuccess => {
                        match job_result.state.maybe_public_bucket_wav_audio_path {
                            None => unreachable!(),
                            Some(public_bucket_wav_audio_path) => {
                                Some(self.tts_output(&public_bucket_wav_audio_path))
                            }
                        }
                    }
                    _ => Some(Err(Error::RequestError(
                        job_result
                            .state
                            .maybe_extra_status_description
                            .unwrap_or("TTS job failed".to_string()),
                    ))),
                },
                Err(e) => Some(Err(e)),
            };

            match status {
                None => std::thread::sleep(std::time::Duration::from_secs(2)),
                Some(result) => return result,
            }
        }
    }
}
