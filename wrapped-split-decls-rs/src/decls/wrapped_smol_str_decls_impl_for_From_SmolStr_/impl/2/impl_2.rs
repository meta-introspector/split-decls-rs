use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < SmolStr > for String { # [inline (always)] fn from (text : SmolStr) -> Self { text . as_str () . into () } }
}