// Gets a list of available voices to use with the TTS API.
// see: https://docs.fakeyou.com/

//! Voices API

use crate::{ApiResult, Error, FakeYou};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use super::TTS_VOICES;

#[derive(Debug, Serialize, Deserialize)]
pub struct TtsListResult {
    /// Whether the request succeeded.
    pub success: bool,
    /// The tts models
    pub models: Vec<TtsModel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TtsModel {
    /// The primary token identifier for the model.
    pub model_token: String,
    /// The type of synthesizer (options: tacotron2, glowtts, etc.)
    pub tts_model_type: String,
    /// The primary token identifier of the user that created the model
    pub creator_user_token: String,
    /// The username of the creator (always lowercase)
    pub creator_username: String,
    /// The display name of the creator (mixed case)
    pub creator_display_name: String,
    /// Gravatar.com hash for the user (if available)
    pub creator_gravatar_hash: String,
    /// Name of the model.
    pub title: String,
    /// IETF BCP 47 language tag.
    pub ietf_language_tag: String,
    /// The primary language tag of the model's speaker / dataset.
    pub ietf_primary_language_subtag: String,
    /// Whether the voice is highlighted on FakeYou.com
    pub is_front_page_featured: bool,
    /// Whether the voice is highlighted on Twitch.
    pub is_twitch_featured: bool,
    /// This is an optional, but guaranteed unique identifier for the voice.
    pub maybe_suggested_unique_bot_command: Option<String>,
    /// Categories this voice belongs to
    pub category_tokens: Vec<String>,
    /// Model upload date
    pub created_at: String,
    /// Model last edit date
    pub updated_at: String,
}

pub trait VoicesApi {
    fn tts_voices(&self) -> ApiResult<TtsListResult>;
}

impl VoicesApi for FakeYou {
    fn tts_voices(&self) -> ApiResult<TtsListResult> {
        let url = format!("{}/{}", &self.api_url, TTS_VOICES);

        let response = self
            .client
            .get(url.as_str())
            .header("Accept", "application/json")
            .send()
            .map_err(|e| Error::RequestFailed(e.to_string()))?;

        match response.status() {
            StatusCode::OK => response
                .json::<TtsListResult>()
                .map_err(|e| Error::ParseError(e.to_string())),
            code => Err(Error::Unknown(code.as_u16())),
        }
    }
}
