pub mod category;
pub mod tts;
pub mod voice;

pub const FAKEYOU_API_URL: &str = "https://api.fakeyou.com";

const TTS_INFERENCE: &str = "tts/inference";
const TTS_JOB: &str = "tts/job";
const TTS_VOICES: &str = "tts/list";
const TTS_CATEGORIES: &str = "category/list/tts";
const STORAGE_URL: &str = "https://storage.googleapis.com/vocodes-public";
