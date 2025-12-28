use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl AsRef < OsStr > for AbsPath { fn as_ref (& self) -> & OsStr { self . 0 . as_ref () } }
}