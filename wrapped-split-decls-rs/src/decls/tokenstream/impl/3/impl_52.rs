use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl DelimSpan { pub fn from_single (sp : Span) -> Self { DelimSpan { open : sp , close : sp } } pub fn from_pair (open : Span , close : Span) -> Self { DelimSpan { open , close } } pub fn dummy () -> Self { Self :: from_single (DUMMY_SP) } pub fn entire (self) -> Span { self . open . with_hi (self . close . hi ()) } }
}