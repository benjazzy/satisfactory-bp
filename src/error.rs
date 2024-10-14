use nom::error::{ErrorKind, FromExternalError, ParseError};
use std::fmt::{Debug, Display, Formatter};
use std::str::Utf8Error;

#[derive(Debug)]
pub enum FgStringError<I> {
    Utf8Error(nom::error::Error<I>, Utf8Error),
    NomError(nom::error::Error<I>),
}

impl<I> From<nom::error::Error<I>> for FgStringError<I> {
    fn from(value: nom::error::Error<I>) -> Self {
        FgStringError::NomError(value)
    }
}

impl<I: Display> Display for FgStringError<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FgStringError::Utf8Error(_, e) => write!(f, "Invalid utf8 in blueprint string: {e}"),
            FgStringError::NomError(e) => write!(f, "Parse error in blueprint string: {e}"),
        }
    }
}

impl<I: Debug + Display> std::error::Error for FgStringError<I> {}

impl<I> ParseError<I> for FgStringError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        FgStringError::NomError(nom::error::Error::from_error_kind(input, kind))
    }

    fn append(input: I, kind: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<I> FromExternalError<I, Utf8Error> for FgStringError<I> {
    fn from_external_error(input: I, kind: ErrorKind, e: Utf8Error) -> Self {
        FgStringError::Utf8Error(nom::error::Error::from_error_kind(input, kind), e)
    }
}
