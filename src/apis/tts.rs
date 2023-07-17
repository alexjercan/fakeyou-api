// Converts text into speech using a voice of your choice and returns audio.
// see: https://docs.fakeyou.com/

//! TTS API

use crate::{ApiResult, FakeYou};
use serde::{Deserialize, Serialize};

use super::{STORAGE_URL, TTS_INFERENCE, TTS_JOB};

fn uuid_idemptency_token_serialize<S>(
    maybe_uuid: &Option<String>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match maybe_uuid {
        Some(uuid) => serializer.serialize_str(uuid),
        None => serializer.serialize_str(uuid::Uuid::new_v4().to_string().as_str()),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InferenceBody {
    /// The voice to use for the inference.
    pub tts_model_token: String,
    /// The text to convert to speech.
    pub inference_text: String,
    /// A random value that can only be used once!
    /// If you don't provide one, we'll generate one for you using uuid v4.
    #[serde(serialize_with = "uuid_idemptency_token_serialize")]
    pub uuid_idempotency_token: Option<String>,
}

impl InferenceBody {
    pub fn new(tts_model_token: &str, inference_text: &str) -> Self {
        Self {
            tts_model_token: tts_model_token.to_string(),
            inference_text: inference_text.to_string(),
            uuid_idempotency_token: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TtsInferenceResult {
    /// Whether the request was successful.
    pub success: bool,
    /// The token to look up the results.
    pub inference_job_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TtsJobResult {
    /// Whether the request succeeded.
    pub success: bool,
    /// Container for the job state record.
    pub state: TtsJobState,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TtsJobState {
    /// Simply returns the same job token you supplied.
    pub job_token: String,
    /// The overall status of the job.
    /// See the docs for more info. https://docs.fakeyou.com/#/?id=poll-tts-request-status
    pub status: TtsJobStatus,
    /// During processing, this may be a human-readable string
    /// to describe the execution status.
    pub maybe_extra_status_description: Option<String>,
    /// The number of attempts we've made to render the audio.
    pub attempt_count: u32,
    /// If there are results, this is the token you'll use to
    /// look up more details (wav file, spectrogram, duration,
    /// execution statistics, etc.)
    pub maybe_result_token: Option<String>,
    /// If there are results, this will show the path to the
    /// wav file. You can use this to avoid looking up the audio
    /// record directly in another API call.
    pub maybe_public_bucket_wav_audio_path: Option<String>,
    /// Voice (tts model) that was used to synthesize the audio.
    pub model_token: String,
    /// The synthesizer architecture
    pub tts_model_type: String,
    /// The name of the model.
    /// This field works the same as the `title` field in the
    /// aforementioned /tts/list request.
    pub title: String,
    /// The text that was used to generate the audio.
    pub raw_inference_text: String,
    /// When the TTS request was made.
    pub created_at: String,
    /// When the job status was last updated.
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TtsJobStatus {
    Pending,
    Started,
    CompleteSuccess,
    CompleteFailure,
    AttemptFailed,
    Dead,
}

#[derive(Debug, Clone)]
pub struct TtsOutputResult {
    /// The raw bytes of the audio in wav format.
    pub bytes: Vec<u8>,
}

pub trait TtsApi {
    fn tts_inference(&self, inference_body: &InferenceBody) -> ApiResult<TtsInferenceResult>;
    fn tts_job(&self, job_id: &str) -> ApiResult<TtsJobResult>;
    fn tts_output(&self, public_bucket_wav_audio_path: &str) -> ApiResult<TtsOutputResult>;
}

impl TtsApi for FakeYou {
    fn tts_inference(&self, voice_settings: &InferenceBody) -> ApiResult<TtsInferenceResult> {
        let voice_settings = serde_json::to_value(voice_settings).unwrap();

        let url = format!("{}/{}", &self.api_url, TTS_INFERENCE);

        self.client
            .post(url.as_str())
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&voice_settings)
            .send()?
            .json::<TtsInferenceResult>()
            .map_err(|e| e.into())
    }

    fn tts_job(&self, job_id: &str) -> ApiResult<TtsJobResult> {
        let url = format!("{}/{}/{}", &self.api_url, TTS_JOB, job_id);

        self.client
            .get(url.as_str())
            .header("Accept", "application/json")
            .send()?
            .json::<TtsJobResult>()
            .map_err(|e| e.into())
    }

    fn tts_output(&self, public_bucket_wav_audio_path: &str) -> ApiResult<TtsOutputResult> {
        let url = format!("{}{}", STORAGE_URL, public_bucket_wav_audio_path);

        let response = self
            .client
            .get(url.as_str())
            .header("Accept", "audio/wav")
            .send()?;

        let bytes = response.bytes()?.to_vec();
        Ok(TtsOutputResult { bytes })
    }
}
