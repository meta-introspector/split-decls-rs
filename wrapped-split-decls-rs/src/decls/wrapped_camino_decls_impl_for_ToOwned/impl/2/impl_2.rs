use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl ToOwned for Utf8Path { type Owned = Utf8PathBuf ; # [inline] fn to_owned (& self) -> Utf8PathBuf { self . to_path_buf () } }
}