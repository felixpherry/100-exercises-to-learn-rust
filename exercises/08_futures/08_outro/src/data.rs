use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Debug, Clone)]
pub enum TicketStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct TicketId(u32);

impl From<u32> for TicketId {
    fn from(id: u32) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone)]
pub struct Ticket {
    id: TicketId,
    title: TicketTitle,
    description: TicketDescription,
    status: TicketStatus,
}

impl Ticket {
    pub fn new(
        id: TicketId,
        title: TicketTitle,
        description: TicketDescription,
        status: TicketStatus,
    ) -> Self {
        Self {
            id,
            title,
            description,
            status,
        }
    }

    pub fn patch(&mut self, ticket_draft: TicketDraft) {
        self.title = ticket_draft.title;
        self.description = ticket_draft.description;
    }

    pub fn id(&self) -> TicketId {
        self.id.clone()
    }

    pub fn title(&self) -> TicketTitle {
        self.title.clone()
    }

    pub fn description(&self) -> TicketDescription {
        self.description.clone()
    }

    pub fn status(&self) -> TicketStatus {
        self.status.clone()
    }
}

pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}
