use std::{
    error::Error,
    fmt::Display,
    num::{ParseFloatError, ParseIntError},
    str::ParseBoolError,
};

use stupid_utils::predule::OptionUnwrapOnNoneError;

/// the error this crate uses
#[derive(Debug)]
pub enum Errors {
    Axum(axum::Error),
    OptionUnwrapOnNoneError,
    WebSocketClosed,
    WrongMessageType(&'static str),
    ParseFloatError(ParseFloatError),
    ParseIntError(ParseIntError),
    ParseBoolError(ParseBoolError),
    InvalidRedstoneLevel(i32),
    InvalidChar(char),
    InvalidPeripheralType(String),
    GPSError(GpsError),
    InvalidSideName(String),
    NoneValue,
}

#[derive(Debug)]
pub enum GpsError {
    Failed,
    Other,
}
impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Errors::Axum(e) => write!(f, "axum error: {}", e),
            Errors::OptionUnwrapOnNoneError => write!(f, "option value is None"),
            Errors::WebSocketClosed => write!(f, "websocket was closed"),
            Errors::WrongMessageType(t) => write!(f, "wrong message type: {}", t),
            Errors::ParseFloatError(e) => e.fmt(f),
            Errors::ParseIntError(e) => e.fmt(f),
            Errors::ParseBoolError(e) => e.fmt(f),
            Errors::InvalidRedstoneLevel(v) => write!(f, "invalid redstone level: {}", v),
            Errors::GPSError(e) => write!(f, "gps error: {:?}", e),
            Errors::NoneValue => write!(f, "got a uncorrect None value"),
            Errors::InvalidSideName(s) => write!(f, "invalid side name: {}", s),
            Errors::InvalidPeripheralType(p) => write!(f, "invalid peripheral type: {}", p),
            Errors::InvalidChar(c) => write!(f, "invalid char(should be ascii only): {}", c),
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

impl From<ParseBoolError> for Errors {
    fn from(value: ParseBoolError) -> Self {
        Self::ParseBoolError(value)
    }
}

impl From<ParseIntError> for Errors {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

pub trait ToErrorsResult {
    type T;
    fn to_errors_result(self) -> Result<Self::T, Errors>;
}

impl<T> ToErrorsResult for Option<T> {
    type T = T;

    fn to_errors_result(self) -> Result<Self::T, Errors> {
        match self {
            Some(v) => Ok(v),
            None => Err(Errors::NoneValue),
        }
    }
}
