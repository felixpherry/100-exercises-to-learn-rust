use crate::data::{Ticket, TicketDraft, TicketId, TicketStatus};
use std::collections::HashMap;

pub struct TicketStore {
    tickets: HashMap<TicketId, Ticket>,
    counter: u32,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: HashMap::new(),
            counter: 1,
        }
    }

    pub fn add_ticket(&mut self, ticket_draft: TicketDraft) -> TicketId {
        let id: TicketId = self.counter.into();
        self.tickets.insert(
            id.clone(),
            Ticket::new(
                id.clone(),
                ticket_draft.title,
                ticket_draft.description,
                TicketStatus::Todo,
            ),
        );

        id
    }

    pub fn get_ticket<T>(&self, id: T) -> Option<Ticket>
    where
        T: Into<TicketId>,
    {
        let id: TicketId = id.into();
        self.tickets.get(&id).cloned()
    }

    pub fn patch_ticket<T>(&mut self, id: T, ticket_draft: TicketDraft)
    where
        T: Into<TicketId>,
    {
        let id: TicketId = id.into();
        if let Some(ticket) = self.tickets.get_mut(&id) {
            ticket.patch(ticket_draft);
        }
    }
}
