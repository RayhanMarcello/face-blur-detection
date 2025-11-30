use axum::{routing::{get}, Router, extract::State, response::IntoResponse};
use axum::Json;
use serde_json::json;
use crate::state::SharedState;
use crate::inference::{preprocess, postprocess};
use crate::errors::{AppError, Result};

pub fn api_router(state: SharedState) -> Router {
    Router::new()
        .route("/api/health", get(health))
        .route("/api/detect", get(detect))
        .with_state(state)
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({"status":"ok"}))
}

async fn detect(State(state): State<SharedState>) -> Result<impl IntoResponse> {
    // Mock detection pipeline
    let frame = state.camera.capture()?;
    let _input = preprocess(&frame)?;
    let detections = state.model.run(&frame)?;
    let results = postprocess(detections)?;
    
    Ok(Json(json!({
        "success": true,
        "num_faces": results.len(),
        "detections": results,
        "mock": true
    })))
}

// Convert errors -> JSON automatically
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(json!({"success": false, "error": self.to_string()}));
        body.into_response()
    }
}
