use std::io::{Error, ErrorKind};
use std::string::ToString;

// Convert others error types to std::io::Error type
pub fn std_error<T, E: ToString>(result: Result<T, E>) -> Result<T, Error>{
    match result {
        Ok(t) => Ok(t),
        Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
    }
}

pub fn invalid_input_error(msg: &str) -> Error {
    return Error::new(ErrorKind::InvalidInput, msg);
}