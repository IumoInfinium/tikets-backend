use axum::{
    extract::State, response::IntoResponse, routing::get, Router
};


use tokio::net::TcpListener;

#[tokio::main]
async fn main(){

    let db = true; 
    let app = Router::new()
        .route("/", get(health_check))
        .with_state(db);

    let server = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(server, app).await.unwrap()
}

async fn health_check(
    State(db) : State<bool>
) -> impl IntoResponse {
    "do stuff".to_string()
}