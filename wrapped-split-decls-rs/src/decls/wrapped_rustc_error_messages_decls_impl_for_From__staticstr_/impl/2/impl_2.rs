use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < & 'static str > for DiagMessage { fn from (s : & 'static str) -> Self { DiagMessage :: Str (Cow :: Borrowed (s)) } }
}