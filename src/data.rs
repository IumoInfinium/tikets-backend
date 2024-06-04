use crate::store::TicketId;

pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: TicketStatus,
}

pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription
}

pub struct TicketTitle(String);
pub struct TicketDescription(String);

// different states a ticket can be on
pub enum TicketStatus {
    Todo,
    Hold,
    InProgress,
    Done,
}

// error information
#[derive(Debug, thiserror::Error)]
pub enum TicketContentError {
    #[error("the field cannot be empty")]
    Empty,
    #[error("the tilte cannot be longer than 50 bytes")]
    TitleTooLong,
    #[error("the description cannot be longer than 500 bytes")]
    DescriptionTooLong
}

impl TryFrom<String> for TicketTitle {
    type Error = TicketContentError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate_title(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<&str> for TicketTitle {
    type Error = TicketContentError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        validate_title(&value)?;
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<String> for TicketDescription {
    type Error = TicketContentError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate_description(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<&str> for TicketDescription {
    type Error = TicketContentError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        validate_description(&value)?;
        Ok(Self(value.to_string()))
    }
}


// validates the title on 3 conditions, with () and fail on Err
fn validate_title(title: &str) -> Result<(), TicketContentError> {
    if title.is_empty() {
        Err(TicketContentError::Empty)
    }
    else if title.len() > 50 {
        Err(TicketContentError::TitleTooLong)
    }
    else {
        Ok(())
    }
}

// validates the description on 3 conditions, with () and fail on Err
fn validate_description(description: &str) -> Result<(), TicketContentError> {
    if description.is_empty() {
        Err(TicketContentError::Empty)
    }
    else if description.len() > 50 {
        Err(TicketContentError::DescriptionTooLong)
    }
    else {
        Ok(())
    }
}