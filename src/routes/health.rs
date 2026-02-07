use crate::types::HealthResponse;
use rocket::serde::json::Json;

#[get("/health")]
pub fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        engines: vec![
            "weasyprint".to_string(),
            "wkhtmltopdf".to_string(),
            "pdflatex".to_string(),
        ],
    })
}
