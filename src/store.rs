use uuid::{Uuid};
use std::collections::BTreeMap;

use crate::data::{Ticket, TicketDraft, TicketStatus};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TicketId(Uuid);

pub struct TicketStore {
    tickets: BTreeMap<TicketId, Ticket>,
    counter: u64,
}

impl TicketStore {
    fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    // give `TicketDraft`, create a new entry in Btree using the created id.
    fn add_ticket(&mut self, ticket_draft: TicketDraft) -> TicketId {
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
    fn get_ticket(&self, ticket_id: TicketId) -> Option<&Ticket> {
        self.tickets.get(&ticket_id)
    }

    fn get_all(&self) -> Vec<Option<&Ticket>> {
        let mut tickets: Vec<Option<&Ticket>> = Vec::new();

        for (k, v) in self.tickets.iter() {
            tickets.push(self.tickets.get(k));
        }
        tickets
    }
}