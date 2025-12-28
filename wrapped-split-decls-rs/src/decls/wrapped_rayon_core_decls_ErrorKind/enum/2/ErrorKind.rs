use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug)] enum ErrorKind { GlobalPoolAlreadyInitialized , CurrentThreadAlreadyInPool , IOError (io :: Error) , }
}