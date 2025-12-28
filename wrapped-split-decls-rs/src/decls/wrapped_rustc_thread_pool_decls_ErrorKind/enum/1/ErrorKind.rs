use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug)] enum ErrorKind { GlobalPoolAlreadyInitialized , IOError (io :: Error) , }
}