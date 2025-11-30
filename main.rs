use tower_http::cors::{CorsLayer, Any};
use axum::http::Method;

mod errors;
mod camera;
mod model;
mod inference;
mod state;
mod routes;

use camera::CameraSource;
use model::YoloModel;
use state::AppState;
use routes::api_router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting Face Detection Server (Mock Mode)...");
    println!("⚠️  Note: Running with mock detection (no real OpenCV/YOLO)");
    println!("   Returns random face boxes for demo purposes\n");

    // Load mock model
    println!("📦 Loading mock model...");
    let model = YoloModel::load("yolo11n.onnx")?;
    println!("✅ Mock model ready.");

    // Mock camera
    println!("📷 Initializing mock camera...");
    let camera = CameraSource::new(0, 640.0, 480.0)?;
    println!("✅ Mock camera ready.");

    let state = AppState::new(camera, model);

    // Build router with CORS for frontend (Vite default port 5173)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = api_router(std::sync::Arc::new(state))
        .layer(cors);

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await?;
    
    println!("\n🌐 Backend server running at http://127.0.0.1:3000");
    println!("🔗 API Endpoints:");
    println!("   GET http://127.0.0.1:3000/api/health");
    println!("   GET http://127.0.0.1:3000/api/detect");
    println!("\n💡 Start frontend: cd frontend && npm run dev");
    println!("   Then open http://localhost:5173\n");

    axum::serve(listener, app).await?;

    Ok(())
}
