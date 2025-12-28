use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a > From < & 'a str > for & 'a Utf8Path { fn from (s : & 'a str) -> & 'a Utf8Path { Utf8Path :: new (s) } }
}