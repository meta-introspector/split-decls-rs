use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl PanicMessage { pub fn into_string (self) -> Option < String > { self . message } }
}