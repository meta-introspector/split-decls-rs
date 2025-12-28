use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl TryFrom < PathBuf > for Utf8PathBuf { type Error = FromPathBufError ; fn try_from (path : PathBuf) -> Result < Utf8PathBuf , Self :: Error > { Utf8PathBuf :: from_path_buf (path) . map_err (| path | FromPathBufError { path , error : FromPathError (()) , }) } }
}