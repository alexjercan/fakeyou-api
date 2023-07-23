// Gets a list of available categories to use with the TTS API.
// see: https://docs.fakeyou.com/

//! Categories API

use crate::{ApiResult, Error, FakeYou};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use super::TTS_CATEGORIES;

#[derive(Debug, Serialize, Deserialize)]
pub struct TtsCategoriesResult {
    /// Whether the request succeeded.
    pub success: bool,
    /// The tts categories
    pub categories: Vec<TtsCategory>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TtsCategory {
    /// The primary token identifier for the category.
    pub category_token: String,
    /// The model type. Other valid values are: "w2l".
    pub model_type: String,
    /// If the category has a parent, this is the parent category's token.
    pub maybe_super_category_token: Option<String>,
    /// If the category can have models in it directly
    pub can_directly_have_models: bool,
    /// If the model can have children categories.
    pub can_have_subcategories: bool,
    /// If the category can only be applied by mods. Typically false.
    pub can_only_mods_apply: bool,
    /// The human readable name of the category
    pub name: String,
    /// This will always be populated.
    pub name_for_dropdown: String,
    /// If the category has been approved by mods for general use.
    pub is_mod_approved: Option<bool>,
    /// When the category was created
    pub created_at: String,
    /// When the category was last edited
    pub updated_at: String,
    /// If the category has been deleted, this will be populated.
    pub deleted_at: Option<String>,
}

pub trait CategoriesApi {
    fn tts_categories(&self) -> ApiResult<TtsCategoriesResult>;
}

impl CategoriesApi for FakeYou {
    fn tts_categories(&self) -> ApiResult<TtsCategoriesResult> {
        let url = format!("{}/{}", &self.api_url, TTS_CATEGORIES);

        let response = self
            .client
            .get(url.as_str())
            .header("Accept", "application/json")
            .send()
            .map_err(|e| Error::RequestFailed(e.to_string()))?;

        match response.status() {
            StatusCode::OK => response
                .json::<TtsCategoriesResult>()
                .map_err(|e| Error::ParseError(e.to_string())),
            code => Err(Error::Unknown(code.as_u16())),
        }
    }
}
