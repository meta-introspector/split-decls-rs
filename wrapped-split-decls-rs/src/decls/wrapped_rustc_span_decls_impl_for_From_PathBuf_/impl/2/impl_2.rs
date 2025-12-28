use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < PathBuf > for FileName { fn from (p : PathBuf) -> Self { FileName :: Real (RealFileName :: LocalPath (p)) } }
}