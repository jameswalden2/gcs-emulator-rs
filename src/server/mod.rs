pub mod handlers;
pub mod models;

use self::handlers::{create_bucket, get_bucket, list_buckets};

use axum::{
    Router,
    routing::{get, post},
};

use std::net::{IpAddr, SocketAddr};

pub async fn run_server(host: &str, port: u16) {
    let parsed_ip: IpAddr = match host.parse() {
        Ok(ip) => ip,
        Err(_) => {
            eprintln!("❌ Error: '{}' is not a valid IP address", host);
            std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/storage/b", get(list_buckets))
        .route("/storage/b/{bucket_name}", get(get_bucket))
        .route("/storage/b", post(create_bucket));

    let addr = SocketAddr::new(parsed_ip, port);
    println!("🚀 Server running at http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
