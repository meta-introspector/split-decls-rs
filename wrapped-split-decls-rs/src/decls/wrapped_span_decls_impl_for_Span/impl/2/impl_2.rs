use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Span { pub fn cover (self , other : Span) -> Span { if self . anchor != other . anchor { return self ; } let range = self . range . cover (other . range) ; Span { range , .. self } } }
}