use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl PartialEq < Symbol > for Path { # [inline] fn eq (& self , name : & Symbol) -> bool { if let [segment] = self . segments . as_ref () && segment == name { true } else { false } } }
}