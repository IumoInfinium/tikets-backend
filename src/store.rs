use uuid::Uuid;
use std::{collections::BTreeMap, path::Display};
use serde::{Serialize, Deserialize};
use std::fmt;
use crate::data::{Ticket, TicketDraft, TicketStatus, TicketTemplate, Errors};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct TicketId(Uuid);

impl fmt::Display for TicketId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// used as a database for all the tickets
#[derive(Clone, Debug, Default)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Ticket>,
    counter: u64,
}

// impl methods used on the database
impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    // give `TicketDraft`, create a new entry in Btree using the created id.
    pub fn add_ticket(&mut self, ticket_draft: TicketDraft) -> TicketId {
        let id = TicketId(Uuid::new_v4());
        let ticket = Ticket {
            id: id,
            title: ticket_draft.title,
            description: ticket_draft.description,
            status: TicketStatus::Todo,
        };
        self.tickets.insert(id, ticket);
        id
    }

    // get the specific ticket of `TicketId`
    pub fn get_ticket(&self, ticket_id: TicketId) -> Option<Ticket> {
        self.tickets.get(&ticket_id).cloned()
    }

    pub fn get_all(&self) -> Vec<Option<Ticket>> {
        let mut tickets: Vec<Option<Ticket>> = Vec::new();

        for (k, v) in self.tickets.iter() {
            tickets.push(self.tickets.get(k).cloned());
        }
        tickets
    }

    pub fn update_ticket(&mut self, ticket_id: TicketId, ticket_template: TicketTemplate) -> Result<Ticket, Errors> {
        match self.tickets.get_mut(&ticket_id) {
            Some(t) => {
                t.title = ticket_template.title.unwrap_or(t.title.clone());
                t.description = ticket_template.description.unwrap_or(t.description.clone());
                t.status = ticket_template.status.unwrap_or(t.status);
                Ok(t.clone())
            },
            _ => Err(Errors::UpdateError),
        }
    }   
}