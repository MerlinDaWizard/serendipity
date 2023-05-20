use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ParsimonyErrors {
    AuthorNoVC,
    RequestError,
    InvalidPermissions,
    NoActiveCall,
    NothingQueued,
}

impl Error for ParsimonyErrors {}

impl Display for ParsimonyErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsimonyErrors::AuthorNoVC => {
                write!(f, "You must be in a voice channel to use this command.")
            }
            ParsimonyErrors::RequestError => {
                write!(f, "Cannot find any music at the given link.")
            }
            ParsimonyErrors::InvalidPermissions => {
                write!(f, "You do not have permission to use this command.")
            }
            ParsimonyErrors::NoActiveCall => {
                write!(f, "There is nothing currently playing.")
            }
            ParsimonyErrors::NothingQueued => {
                write!(
                    f,
                    "You cannot do this action as there is nothing currently queued."
                )
            }
        }
    }
}
