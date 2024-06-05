use crate::store::TicketId;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: TicketStatus,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct TicketTitle(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct TicketDescription(pub String);

// different states a ticket can be on

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
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