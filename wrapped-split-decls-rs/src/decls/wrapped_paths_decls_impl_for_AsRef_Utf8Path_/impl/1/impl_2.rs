use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl AsRef < Utf8Path > for RelPath { fn as_ref (& self) -> & Utf8Path { & self . 0 } }
}