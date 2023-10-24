use std::{error::Error, fmt::Display, num::ParseFloatError};

use stupid_utils::predule::OptionUnwrapOnNoneError;

#[derive(Debug)]
pub enum Errors {
    Axum(axum::Error),
    OptionUnwrapOnNoneError,
    WebSocketClosed,
    WrongMessageType(&'static str),
    ParseFloatError(ParseFloatError),
    InvalidRedstoneLevel(i32),
}
impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Errors::Axum(e) => write!(f, "axum error: {}", e),
            Errors::OptionUnwrapOnNoneError => write!(f, "option value is None"),
            Errors::WebSocketClosed => write!(f, "websocket was closed"),
            Errors::WrongMessageType(t) => write!(f, "wrong message type: {}", t),
            Errors::ParseFloatError(e) => e.fmt(f),
            Errors::InvalidRedstoneLevel(v) => write!(f, "invalid redstone level: {}", v),
        }
    }
}

impl Error for Errors {}
impl From<axum::Error> for Errors {
    fn from(value: axum::Error) -> Self {
        Self::Axum(value)
    }
}
impl From<OptionUnwrapOnNoneError> for Errors {
    fn from(_value: OptionUnwrapOnNoneError) -> Self {
        Self::OptionUnwrapOnNoneError
    }
}
impl From<ParseFloatError> for Errors {
    fn from(value: ParseFloatError) -> Self {
        Self::ParseFloatError(value)
    }
}
