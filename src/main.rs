use std::path::PathBuf;

use atom_services::{Router, ServiceInstance};

#[tokio::main]
async fn main() {
    let path = PathBuf::from(std::env::var("CONFIG").expect("env CONFIG not set"));
    let instance = ServiceInstance::load(&path);
    let port = instance.config.port;
    let app = axum::Router::new().nest("/api/perms/v1", Router::get(instance));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}",))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
