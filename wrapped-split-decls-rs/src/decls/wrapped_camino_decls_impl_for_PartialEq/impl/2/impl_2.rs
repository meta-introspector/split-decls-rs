use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl PartialEq for Utf8Path { # [inline] fn eq (& self , other : & Utf8Path) -> bool { self . components () . eq (other . components ()) } }
}