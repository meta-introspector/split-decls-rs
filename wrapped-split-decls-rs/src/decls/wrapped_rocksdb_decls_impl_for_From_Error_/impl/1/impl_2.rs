use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < Error > for String { fn from (e : Error) -> String { e . message } }
}