use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::net::TcpListener;
use tracing::info;

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("Serving {:?} on port {}", &path, port);

    let state = HttpServeState { path };
    let router = Router::new()
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    info!("Requesting file {:?}", path);
    let path = state.path.as_path().join(path);
    if path.exists() {
        // read the file and return it's content
        match tokio::fs::read_to_string(&path).await {
            Ok(content) => (StatusCode::OK, content),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error reading file {:?}: {:?}", path, e),
            ),
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            format!("File {:?} does not exist", path),
        )
    }
}
