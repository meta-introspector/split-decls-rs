use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl AsRef < Path > for Utf8PathBuf { # [inline] fn as_ref (& self) -> & Path { & self . 0 } }
}