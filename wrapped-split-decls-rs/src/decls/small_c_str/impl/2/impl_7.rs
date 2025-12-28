use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Deref for SmallCStr { type Target = ffi :: CStr ; # [inline] fn deref (& self) -> & ffi :: CStr { self . as_c_str () } }
}