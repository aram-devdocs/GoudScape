use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use hyper::Response;
use serde_json::json;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower::ServiceExt;

use super::super::main::{update_cube_movement, AppState, CubeMovement};

#[tokio::test]
async fn test_update_cube_movement() {
    let app_state = Arc::new(Mutex::new(AppState::default()));
    let app = Router::new()
        .route("/update_cube_movement", axum::routing::post(update_cube_movement))
        .layer(axum::AddExtensionLayer::new(app_state.clone()));

    let payload = json!({
        "x": 5.0,
        "y": 10.0,
    });

    let response = app
        .oneshot(
            Request::builder()
                .uri("/update_cube_movement")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let state = app_state.lock().await;
    assert_eq!(state.cube_position, (5.0, 10.0));
}
