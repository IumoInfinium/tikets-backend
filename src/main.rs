pub mod store;
pub mod data;

use std::sync::Arc;
use axum::{
    extract::{Path, State}, http::StatusCode, response::{IntoResponse, Response}, routing::{get, post}, Json, Router
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use store::{TicketId, TicketStore};
use data::{
    identify_status, Errors, TicketDescription, TicketDraft, TicketTemplate, TicketTitle
};

// Safing the ticketstore instance over the threads using ref count
type TicketStoreState = Arc<Mutex<TicketStore>>;

// acts as a collection box for user input for `CREATE` & `PATCH`
#[derive(Debug, Deserialize, Serialize)]
struct TicketDraftInput {
    title: Option<String>,
    description: Option<String>,
    status: Option<String>
}

#[tokio::main]
async fn main(){
    
    let db_state = TicketStoreState::default();

    // test ticket 
    db_state.lock().await.add_ticket(
        TicketDraft {
            title: TicketTitle("Neil's Client Meeting".into()),
            description: TicketDescription("Complete the work for the meeting presentation".into())
        }
    );

    // lists of routes
    let app = Router::new()
        .route("/status", get(health_check))
        .route("/", get(get_all_tickets))
        .route("/ticket", post(create_ticket))
        .route("/ticket/:id", get(get_ticket).patch(update_ticket))
        .with_state(db_state);

    // TCP server creation and binding it to axum routing service
    let server = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(server, app).await.unwrap()
}

// server state check
async fn health_check(
) -> impl IntoResponse {
    "OK".to_string()
}

// get the list of all the tickets
async fn get_all_tickets(
    State(db_state): State<TicketStoreState>
) -> impl IntoResponse {
    let tickets = db_state.lock().await.get_all();
    Json(tickets)
}

// get the ticket information using the `TicketId`
async fn get_ticket(
    Path(ticket_id): Path<TicketId>,
    State(db_state) : State<TicketStoreState>
) -> impl IntoResponse {
    Json(db_state.lock().await.get_ticket(ticket_id))
}

// Creates a new ticket
async fn create_ticket(
    State(db_state): State<TicketStoreState>,
    Json(input) : Json<TicketDraftInput>,
) -> Response {
    // firstly create temporary ticket
    let draft_ticket: TicketDraft = TicketDraft {
        title: TicketTitle(input.title.unwrap_or("".into())),
        description: TicketDescription(input.description.unwrap_or("".into())),
    };
    // create a new entry in the database
    let ticket_id =  db_state.lock().await.add_ticket(draft_ticket);

    Json(db_state.lock().await.get_ticket(ticket_id)).into_response()
}

async fn update_ticket(
    Path(ticket_id): Path<TicketId>,
    State(db_state) : State<TicketStoreState>,
    Json(ticket_input) : Json<TicketDraftInput>,
) -> Response {

    let ticket = db_state.lock().await.get_ticket(ticket_id);

    // in case no ticket is found, response as failed
    let Some(_) = ticket else {
        let mut out = Json(Errors::UpdateError.to_string()).into_response();
        *out.status_mut() = StatusCode::NOT_FOUND;
        return out
    };

    // extract `title`, 'description' and convert `status` to appropriate type of Options
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
            _ => return Json(Errors::StatusUnParseable.to_string()).into_response()
    };

    let template = TicketTemplate{ title, description, status };

    // update the ticket 
    let update_state = db_state.lock().await.update_ticket(ticket_id, template);

    // verify and respond back
    match update_state {
        Ok(ticket) => Json(ticket).into_response(),
        _ => {
            let mut out = Json(Errors::UpdateError.to_string()).into_response();
            *out.status_mut() = StatusCode::BAD_REQUEST;
            out
        }
    }
}
