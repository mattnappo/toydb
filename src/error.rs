use actix::MailboxError;
use jsonrpc_v2::ErrorLike;
use prost::{DecodeError, EncodeError};
use sqlparser::parser::ParserError;
use std::{error::Error as StdError, fmt};
use tokio::io::Error as IoError;
use xdg::BaseDirectoriesError;

#[derive(Debug)]
pub enum Error {
    XdgError(BaseDirectoriesError),
    IoError(IoError),
    MailboxError(MailboxError),
    PageOutOfBounds,
    MutexError,
    ConversionError,
    RecordNotFound,
    TraversalError,
    DecodeError(DecodeError),
    EncodeError(EncodeError),
    MiscDecodeError,
    InvalidKey,
    MultiplePrimaryKeyClauses,
    MissingCatalogueEntry,
    InvalidCondition,
    JoinColumnNotFound,
    SqlParserError(ParserError),
    Unimplemented(Option<String>),
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Self::IoError(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::XdgError(e) => write!(
                f,
                "encountered an error while resolving base XDG paths: {:?}",
                e
            ),
            Self::IoError(e) => write!(f, "encountered an IO error: {:?}", e),
            Self::MailboxError(e) => write!(f, "encountered an actor mailbox error: {:?}", e),
            Self::PageOutOfBounds => write!(f, "the record is larger than a page"),
            Self::MutexError => write!(f, "encountered a mutex error"),
            Self::ConversionError => write!(f, "encountered a type conversion error"),
            Self::RecordNotFound => write!(f, "the record was not found in the page"),
            Self::TraversalError => write!(f, "failed to correctly traverse a tree structure"),
            Self::DecodeError(e) => write!(f, "failed to decode a message with protobuf: {:?}", e),
            Self::EncodeError(e) => write!(f, "failed to encode a message with protobuf: {:?}", e),
            Self::MiscDecodeError => write!(f, "failed to parse the message"),
            Self::InvalidKey => write!(f, "the value is not a valid key value"),
            Self::MultiplePrimaryKeyClauses => {
                write!(f, "the query included multiple primary key clauses")
            }
            Self::MissingCatalogueEntry => write!(f, "missing catalogue entry"),
            Self::InvalidCondition => write!(f, "invalid condition"),
            Self::JoinColumnNotFound => write!(f, "join column not found"),
            Self::SqlParserError(e) => write!(f, "encountered an error while parsing SQL: {:?}", e),
            Self::Unimplemented(feature) => write!(
                f,
                "this feature of SQL is not implemented yet: {}",
                feature.clone().unwrap_or(String::from("unknown feature"))
            ),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::XdgError(e) => Some(e),
            Self::IoError(e) => Some(e),
            Self::MailboxError(e) => Some(e),
            Self::DecodeError(e) => Some(e),
            Self::EncodeError(e) => Some(e),
            Self::SqlParserError(e) => Some(e),
            Self::PageOutOfBounds
            | Self::MutexError
            | Self::ConversionError
            | Self::RecordNotFound
            | Self::TraversalError
            | Self::MiscDecodeError
            | Self::InvalidKey
            | Self::MultiplePrimaryKeyClauses
            | Self::MissingCatalogueEntry
            | Self::InvalidCondition
            | Self::JoinColumnNotFound
            | Self::Unimplemented(_) => None,
        }
    }
}

impl ErrorLike for Error {
    fn code(&self) -> i64 {
        match self {
            Self::XdgError(_) => 0,
            Self::IoError(_) => 1,
            Self::MailboxError(_) => 2,
            Self::PageOutOfBounds => 3,
            Self::MutexError => 4,
            Self::ConversionError => 5,
            Self::RecordNotFound => 6,
            Self::TraversalError => 7,
            Self::DecodeError(_) => 8,
            Self::EncodeError(_) => 9,
            Self::MiscDecodeError => 10,
            Self::InvalidKey => 11,
            Self::MultiplePrimaryKeyClauses => 12,
            Self::MissingCatalogueEntry => 13,
            Self::InvalidCondition => 14,
            Self::JoinColumnNotFound => 15,
            Self::SqlParserError(_) => 16,
            Self::Unimplemented(_) => 17,
        }
    }
}
