use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl PartialEq < String > for SmolStr { # [inline (always)] fn eq (& self , other : & String) -> bool { self . as_str () == other } }
}