use actix_web::{get, HttpResponse};

#[get("/check")]
pub async fn check_action() -> HttpResponse {
    info!("GET /check");
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"status": "ok"}"#)
}
