//! # Render hook
//!
//! This module expose the function to call the render hook

use base64::engine::general_purpose;
use base64::Engine as _;

use super::HookResult;

pub struct RenderHook {
    url: String,
}

impl From<String> for RenderHook {
    fn from(url: String) -> Self {
        Self { url }
    }
}

impl RenderHook {
    /// Call hook
    pub async fn call(
        &self,
        rendered_template: &str,
        metadata: Option<serde_json::Value>,
    ) -> HookResult<()> {
        let body = general_purpose::STANDARD.encode(rendered_template.as_bytes());
        let payload = Payload::new(body, metadata);

        debug!("Calling render hook at {}", self.url);
        reqwest::Client::new()
            .post(&self.url)
            .json(&payload)
            .send()
            .await?;
        info!("Render Hook returned 200!");

        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct Payload {
    body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<serde_json::Value>,
}

impl Payload {
    pub fn new(body: String, metadata: Option<serde_json::Value>) -> Self {
        Self { body, metadata }
    }
}
