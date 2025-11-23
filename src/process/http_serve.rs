use anyhow::Result;
use axum::Router;
use axum::extract::{Path, State};
use axum::routing::get;
use std::net::SocketAddr;
use std::path::{PathBuf};
use std::sync::Arc;
use axum::http::StatusCode;
use tracing::info;

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}
pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", addr, port);
    let state = HttpServeState { path };
    let router = Router::new()
        .route("/{*path}", get(file_handler))
        .with_state(Arc::new(state));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(State(state): State<Arc<HttpServeState>>, Path(path):Path<String>) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("ReadingFile{:?}",p);
    if !p.exists() {
        (StatusCode::NOT_FOUND, format!("File {} not found", p.display()))
    }else {
        match tokio::fs::read_to_string(p).await{
            Ok(content)=>{
                info!("Read {} Bytes",content.len());
                (StatusCode::OK, content)
            }
            Err(e)=>{
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Error reading file: {}", e))
            }
        }
    }
}
