pub mod store;
pub mod data;

use axum::{
    extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router
};
use serde::Deserialize;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use std::sync::Arc;

use store::{TicketId, TicketStore};
use data::{Ticket, TicketContentError, TicketDescription, TicketDraft, TicketTitle};

type TicketStoreState = Arc<Mutex<TicketStore>>;

#[tokio::main]
async fn main(){
    
    let db_state = TicketStoreState::default();

    db_state.lock().await.add_ticket(
        TicketDraft {
            title: TicketTitle("Neil's Client Meeting".into()),
            description: TicketDescription("Complete the work for the meeting presentation".into())
        }
    );
    let app = Router::new()
        .route("/status", get(health_check))
        .route("/", get(get_all_tickets))
        .route("/ticket", post(create_ticket))
        .route("/ticket/:id", get(get_ticket))
        .with_state(db_state);

    let server = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(server, app).await.unwrap()
}

async fn health_check(
) -> impl IntoResponse {
    "server is healthy".to_string()
}

async fn get_all_tickets(
    State(db_state): State<TicketStoreState>
) -> impl IntoResponse {
    let tickets = db_state.lock().await.get_all();
    Json(tickets)
}

async fn get_ticket(
    Path(ticket_id): Path<TicketId>,
    State(db_state) : State<TicketStoreState>
) -> impl IntoResponse {
    Json(db_state.lock().await.get_ticket(ticket_id))
}

#[derive(Debug, Deserialize)]
struct TicketDraftInput {
    title: Option<String>,
    description: Option<String>,
}
async fn create_ticket(
    State(db_state): State<TicketStoreState>,
    Json(input) : Json<TicketDraftInput>,
) -> impl IntoResponse {


    let draft_ticket: TicketDraft = TicketDraft {
        title: TicketTitle(input.title.unwrap_or("".into())),
        description: TicketDescription(input.description.unwrap_or("".into()))
    };
    let ticket_id =  db_state.lock().await.add_ticket(draft_ticket);

    (StatusCode::CREATED, Json(ticket_id))
}
