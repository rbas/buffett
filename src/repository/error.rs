use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct FetchError {
    message: Option<String>,
}

impl FetchError {
    pub fn new(message: String) -> Self {
        FetchError {
            message: Some(message),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SaveError {}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Cannot fetch data. {:#?}",
            self.message.clone().unwrap_or(String::from(""))
        )
    }
}
impl error::Error for FetchError {}

impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot save data")
    }
}
impl error::Error for SaveError {}
