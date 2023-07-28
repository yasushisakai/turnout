use std::{fmt::Display, error::Error};

#[derive(Debug)]
pub enum TurnoutError{
    AddressConversion
}

impl Display for TurnoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self  {
            TurnoutError::AddressConversion => write!(f, "could not convert string to sha256 hex")
        }
    }
}


impl Error for TurnoutError{
    fn description(&self) -> &str {
        match *self{
            TurnoutError::AddressConversion => "could not convert string to sha 256 hex",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            TurnoutError::AddressConversion => None,
        } 
    }
}
