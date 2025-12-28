use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl TryFrom < & str > for RelPathBuf { type Error = Utf8PathBuf ; fn try_from (path : & str) -> Result < RelPathBuf , Utf8PathBuf > { RelPathBuf :: try_from (Utf8PathBuf :: from (path)) } }
}