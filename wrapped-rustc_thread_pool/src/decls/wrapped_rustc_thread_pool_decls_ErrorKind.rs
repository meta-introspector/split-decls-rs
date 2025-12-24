use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug)]
enum ErrorKind {
    GlobalPoolAlreadyInitialized,
    IOError(io::Error),
}
