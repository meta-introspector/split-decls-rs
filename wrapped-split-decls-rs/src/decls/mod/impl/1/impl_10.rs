use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Drop for ThreadPool { fn drop (& mut self) { self . registry . terminate () ; } }
}