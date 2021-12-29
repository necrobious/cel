use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CelError {
    details: String
}

impl CelError {
    pub fn new(msg: &str) -> CelError {
        CelError {details: msg.to_string()}
    }
}

impl fmt::Display for CelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for CelError {
    fn description(&self) -> &str {
        &self.details
    }
}