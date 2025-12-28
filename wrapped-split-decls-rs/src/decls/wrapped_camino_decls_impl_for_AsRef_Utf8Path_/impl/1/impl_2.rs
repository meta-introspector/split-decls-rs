use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl AsRef < Utf8Path > for String { # [inline] fn as_ref (& self) -> & Utf8Path { Utf8Path :: new (self) } }
}