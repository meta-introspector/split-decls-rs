use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < Box < Utf8Path > > for Utf8PathBuf { fn from (path : Box < Utf8Path >) -> Utf8PathBuf { path . into_path_buf () } }
}