pub mod store;
pub mod data;

use axum::{
    extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use std::sync::Arc;

use store::{TicketId, TicketStore};
use data::{
    identify_status,
    Ticket, TicketTitle, TicketDescription, TicketStatus,
    TicketDraft, TicketTemplate,
    TicketContentError
};

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
        .route("/ticket/:id", get(get_ticket).post(update_ticket))
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

#[derive(Debug, Deserialize, Serialize)]
struct TicketDraftInput {
    title: Option<String>,
    description: Option<String>,
    status: Option<String>
}
async fn create_ticket(
    State(db_state): State<TicketStoreState>,
    Json(input) : Json<TicketDraftInput>,
) -> impl IntoResponse {

    let draft_ticket: TicketDraft = TicketDraft {
        title: TicketTitle(input.title.unwrap_or("".into())),
        description: TicketDescription(input.description.unwrap_or("".into())),
    };
    let ticket_id =  db_state.lock().await.add_ticket(draft_ticket);

    (StatusCode::CREATED, Json(db_state.lock().await.get_ticket(ticket_id)))
}

async fn update_ticket(
    Path(ticket_id): Path<TicketId>,
    State(db_state) : State<TicketStoreState>,
    Json(ticket_input) : Json<TicketDraftInput>,
) -> (StatusCode, impl IntoResponse) {

    let ticket = db_state.lock().await.get_ticket(ticket_id);
    let Some(t) = ticket else {
        return (StatusCode::NOT_FOUND, Json(format!("Failed to find the ticket : {ticket_id}")))
    };
    let title = match ticket_input.title {
            Some(t) => Some(TicketTitle(t)),
            _ => None
    };
    let description = match ticket_input.description {
            Some(d) => Some(TicketDescription(d)),
            _ => None
    };
    let status = match identify_status(ticket_input.status.unwrap()) {
            Ok(d) => Some(d),
            _ => None
    };
    let template = TicketTemplate{
        title: title,
        description: description,
        status: status
    };

    db_state.lock().await.update_ticket(ticket_id, template);

    (StatusCode::OK, Json(format!("Updated to find the ticket : {ticket_id}")))

}
