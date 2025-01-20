use axum::{
    routing::post,
    Router,
    Json,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::sync::oneshot;
use tokio::task;
use std::sync::{Arc, Mutex};
use networking::networking::{Networking, CubeMovement};

#[derive(Deserialize)]
struct CubeMovement {
    x: f32,
    y: f32,
}

#[tokio::test]
async fn test_client_server_communication() {
    // Setup server
    let server_addr = "127.0.0.1:3000".parse::<SocketAddr>().unwrap();
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let server_handle = task::spawn(async move {
        let app_state = Arc::new(Mutex::new(AppState::default()));
        let app = Router::new()
            .route("/update_cube_movement", post(update_cube_movement))
            .layer(axum::AddExtensionLayer::new(app_state));
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        let server = axum::Server::bind(&addr).serve(app.into_make_service());
        tokio::select! {
            _ = server => {},
            _ = shutdown_rx => {},
        }
    });

    // Setup client
    let mut app = App::build();
    app.add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(move_cube.system())
        .add_system(send_cube_movement.system())
        .add_system(exit_after_movement.system());

    // Run client
    app.run();

    // Shutdown server
    shutdown_tx.send(()).unwrap();
    server_handle.await.unwrap();
}

fn exit_after_movement(
    query: Query<(&Cube, &Transform)>,
    mut exit: EventWriter<AppExit>,
) {
    for (_cube, transform) in query.iter() {
        if transform.translation != Vec3::ZERO {
            exit.send(AppExit);
        }
    }
}

async fn update_cube_movement(
    Json(payload): Json<CubeMovement>,
    axum::Extension(state): axum::Extension<Arc<Mutex<AppState>>>,
) {
    let mut state = state.lock().await;
    state.cube_position = (payload.x, payload.y);
}

#[derive(Default)]
struct AppState {
    cube_position: (f32, f32),
}
