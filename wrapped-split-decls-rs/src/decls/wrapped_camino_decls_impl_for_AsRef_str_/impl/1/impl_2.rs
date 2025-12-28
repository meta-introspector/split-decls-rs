use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl AsRef < str > for Utf8PathBuf { # [inline] fn as_ref (& self) -> & str { self . as_str () } }
}