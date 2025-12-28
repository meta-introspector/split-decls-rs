use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < F : FnOnce () > OnDrop < F > { # [doc = " Disables on-drop call."] # [inline] pub fn disable (mut self) { self . 0 . take () ; } }
}