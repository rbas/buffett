use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct FetchError {}

#[derive(Debug, Clone)]
pub struct SaveError {}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot fetch data")
    }
}
impl error::Error for FetchError {}

impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot save data")
    }
}
impl error::Error for SaveError {}
