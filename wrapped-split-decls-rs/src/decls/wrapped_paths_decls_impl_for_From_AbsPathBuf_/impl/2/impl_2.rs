use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < AbsPathBuf > for PathBuf { fn from (AbsPathBuf (path_buf) : AbsPathBuf) -> PathBuf { path_buf . into () } }
}