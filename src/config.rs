//! # Config
//!
//! App configuration

/// App configuration read from environment
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// configurable url to send rendered template according to tokyo-draft hook API
    pub render_hook: Option<String>,
    /// directory containing templates to render
    pub templates_dir: String,
    /// port the web server listens to
    pub web_port: u16,
}

impl Config {
    pub fn try_from_env() -> anyhow::Result<Self> {
        envy::from_env()
            .map_err(|e| anyhow::anyhow!("could not load config from environment: {}", e))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn should_parse_config_from_env() {
        assert!(Config::try_from_env().is_ok());
    }
}
