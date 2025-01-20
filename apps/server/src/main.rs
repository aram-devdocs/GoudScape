use axum::{
    routing::post,
    Router,
    Json,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::sync::Mutex;
use std::sync::Arc;
use networking::networking::{Networking, CubeMovement};

#[derive(Deserialize)]
struct CubeMovement {
    x: f32,
    y: f32,
}

#[tokio::main]
async fn main() {
    let app_state = Arc::new(Mutex::new(AppState::default()));

    let app = Router::new()
        .route("/update_cube_movement", post(update_cube_movement))
        .layer(axum::AddExtensionLayer::new(app_state));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Default)]
struct AppState {
    cube_position: (f32, f32),
}

async fn update_cube_movement(
    Json(payload): Json<CubeMovement>,
    axum::Extension(state): axum::Extension<Arc<Mutex<AppState>>>,
) {
    let mut state = state.lock().await;
    state.cube_position = (payload.x, payload.y);
}
