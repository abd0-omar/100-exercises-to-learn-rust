use axum::{
    extract::Path,
    routing::{get, patch, post},
    Router,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler0))
        .route("/tickets/:id", get(get_tickets))
        .route("/tickets", post(create_ticket))
        .route("/tickets", patch(patch_ticket));

    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    println!("Going Live On: {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn handler0() -> &'static str {
    "hello world"
}

async fn get_tickets(Path(id): Path<String>) -> String {
    format!("get tickets, {}", id)
}
async fn create_ticket() -> &'static str {
    "make ticket"
}
async fn patch_ticket() -> &'static str {
    "update patch ticket"
}
