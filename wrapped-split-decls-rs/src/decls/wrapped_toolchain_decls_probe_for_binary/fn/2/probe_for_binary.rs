use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn probe_for_binary (path : Utf8PathBuf) -> Option < Utf8PathBuf > { let with_extension = match env :: consts :: EXE_EXTENSION { "" => None , it => Some (path . with_extension (it)) , } ; iter :: once (path) . chain (with_extension) . find (| it | it . is_file ()) }
}