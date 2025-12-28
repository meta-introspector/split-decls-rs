use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl LexError { pub fn span (& self) -> Span { Span :: _new (self . inner . span ()) } }
}