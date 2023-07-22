use std::collections::HashMap;

use actix_web::http::header::ContentType;
use actix_web::{post, web, Error, HttpResponse};

use super::WebserverData;
use crate::render::RenderError;

#[derive(Debug, Deserialize)]
pub struct Payload {
    template: String,
    data: HashMap<String, serde_json::Value>,
    #[serde(rename = "hookMetadata")]
    hook_metadata: Option<serde_json::Value>,
}

#[post("/render")]
pub async fn render_action(
    payload: web::Json<Payload>,
    data: web::Data<WebserverData>,
) -> Result<HttpResponse, Error> {
    info!("POST /render");
    debug!("got template {}", payload.template);

    let payload = payload.into_inner();

    debug!("rendering template...");
    let rendered_template = match data.render_service.render(&payload.template, payload.data) {
        Ok(body) => body,
        Err(e) => {
            return Ok(match e {
                RenderError::TemplateNotFound(name) => {
                    error!("unknown template '{name}'");
                    HttpResponse::BadRequest().body(format!("unknown template: {name}"))
                }
                RenderError::Template(e) => {
                    error!("failed to render template: {e}");
                    HttpResponse::InternalServerError().body("failed to render template")
                }
            })
        }
    };

    if let Some(hook) = data.render_hook.as_ref() {
        debug!("render hook is set, sending out data...");
        if let Err(e) = hook.call(&rendered_template, payload.hook_metadata).await {
            error!("failed to call render hook: {e}");
            return Ok(HttpResponse::InternalServerError().body("failed to call hook"));
        }
    }

    info!("POST /render OK");

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered_template))
}
