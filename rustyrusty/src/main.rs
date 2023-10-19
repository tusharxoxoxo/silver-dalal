use std::net::SocketAddr;

use axum::{routing::get, Router};

// basic handler that responds with a static string
async fn index() -> &'static str {
	"Index"
}

async fn weather() -> &'static str {
	"Weather"
}

async fn stats() -> &'static str {
	"Stats"
}

#[tokio::main]
async fn main() {
	let app = Router::new()
    	.route("/", get(index))
    	.route("/balance", get(weather))
    	.route("/stats", get(stats));

	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	axum::Server::bind(&addr)
    	.serve(app.into_make_service())
    	.await
    	.unwrap();
}
