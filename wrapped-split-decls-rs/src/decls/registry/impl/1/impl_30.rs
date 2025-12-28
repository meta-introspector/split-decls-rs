use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a > Drop for Terminator < 'a > { fn drop (& mut self) { self . 0 . terminate () } }
}