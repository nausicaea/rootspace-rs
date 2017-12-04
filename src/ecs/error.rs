use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum EcsError {
    ComponentNotFound(String),
    MultipleComponentsFound(String),
}

impl Error for EcsError {
    fn description(&self) -> &str {
        use self::EcsError::*;
        match *self {
            ComponentNotFound(_) => "The specified component was not found",
            MultipleComponentsFound(_) => "The specified component was found more than once",
        }
    }
    fn cause(&self) -> Option<&Error> {
        use self::EcsError::*;
        match *self {
            ComponentNotFound(_) => None,
            MultipleComponentsFound(_) => None,
        }
    }
}

impl fmt::Display for EcsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::EcsError::*;
        match *self {
            ComponentNotFound(ref s) => write!(f, "Could not find the components '{}'", s),
            MultipleComponentsFound(ref s) => write!(f, "The components '{}' were found more than once", s),
        }
    }
}
