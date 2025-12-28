use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Span { # [doc = " Execute `f` in with this span active, consuming it."] pub fn into_scope < T > (self , f : impl FnOnce () -> T) -> T { f () } }
}