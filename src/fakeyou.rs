use serde::{Deserialize, Serialize};
use ureq::{Agent, AgentBuilder};

use crate::FAKEYOU_API_URL;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Auth {
    pub api_key: Option<String>,
}

impl Auth {
    pub fn new(api_key: &str) -> Auth {
        Auth {
            api_key: Some(api_key.to_string()),
        }
    }

    pub fn from_env() -> Result<Self, String> {
        let api_key =
            std::env::var("FAKEYOU_API_KEY").map_err(|_| "Missing FAKEYOU_API_KEY".to_string())?;
        Ok(Self {
            api_key: Some(api_key),
        })
    }
}

#[derive(Clone, Debug)]
pub struct FakeYou {
    pub auth: Auth,
    pub api_url: String,
    pub(crate) agent: Agent,
}

impl Default for FakeYou {
    fn default() -> Self {
        FakeYou::new(Auth::default(), FAKEYOU_API_URL)
    }
}

impl FakeYou {
    pub fn new(auth: Auth, api_url: &str) -> FakeYou {
        FakeYou {
            auth,
            api_url: api_url.to_string(),
            agent: AgentBuilder::new().build(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test_fakeyou() {
        let auth = Auth::default();
        let fakeyou = FakeYou::new(auth, "https://api.fakeyou.com/");

        assert_eq!(fakeyou.api_url, "https://api.fakeyou.com/");
    }
}
