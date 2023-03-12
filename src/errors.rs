use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum ErrorKind {
    CoordinatesOutsideBoard
}

impl ErrorKind {
    fn to_str(&self) -> &'static str {
        return match self {
            ErrorKind::CoordinatesOutsideBoard => "coordinates are outside legal board boundaries"
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad(self.to_str())
    }
}