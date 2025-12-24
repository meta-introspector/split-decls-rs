use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NonStringLiteral => f.write_str("expected string literal"),
            Error::UuidParse(_, ref e) => write!(f, "{}", e),
        }
    }
}
