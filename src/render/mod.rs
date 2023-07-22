use serde::ser::Serialize;
use tera::{Context, Tera};
use thiserror::Error;

/// A service which renders HTML templates. Based on Tera.
#[derive(Clone)]
pub struct RenderService {
    tera: Tera,
}

type RenderResult<T> = Result<T, RenderError>;

#[derive(Error, Debug)]
pub enum RenderError {
    #[error("Template error: {0}")]
    Template(tera::Error),
    #[error("Template {0} not found")]
    TemplateNotFound(String),
}

impl From<tera::Error> for RenderError {
    fn from(e: tera::Error) -> Self {
        match e.kind {
            tera::ErrorKind::TemplateNotFound(name) => Self::TemplateNotFound(name),
            _ => Self::Template(e),
        }
    }
}

impl TryFrom<&str> for RenderService {
    type Error = RenderError;

    fn try_from(templates_dir: &str) -> Result<Self, Self::Error> {
        debug!("starting render service from {templates_dir}");
        Tera::new(templates_dir)
            .map_err(RenderError::from)
            .map(RenderService::from)
    }
}

impl From<Tera> for RenderService {
    fn from(tera: Tera) -> Self {
        Self { tera }
    }
}

impl RenderService {
    /// render template by name using provided data
    pub fn render(
        &self,
        template_name: &str,
        data: impl Serialize + std::fmt::Debug,
    ) -> RenderResult<String> {
        debug!("rendering template {template_name} with data {data:?}");
        self.tera
            .render(template_name, &Context::from_serialize(data)?)
            .map_err(RenderError::from)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::config::Config;

    #[test]
    fn should_init_render_service() {
        let config = Config::try_from_env().unwrap();
        assert!(RenderService::try_from(config.templates_dir.as_str()).is_ok());
    }

    #[test]
    fn should_render_template() {
        let config = Config::try_from_env().unwrap();
        let first_name = "Omar".to_string();
        let payload = Payload { first_name };

        let service = RenderService::try_from(config.templates_dir.as_str()).unwrap();
        assert!(service.render("hello-world.html", payload).is_ok());
    }

    #[test]
    fn should_return_unexisting_template() {
        let config = Config::try_from_env().unwrap();
        let first_name = "Omar".to_string();
        let payload = Payload { first_name };

        let service = RenderService::try_from(config.templates_dir.as_str()).unwrap();
        assert!(matches!(
            service.render("404.html", payload).unwrap_err(),
            RenderError::TemplateNotFound(_)
        ));
    }

    #[derive(Deserialize, Serialize, Debug)]
    struct Payload {
        first_name: String,
    }
}
