use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [allow (dead_code)] pub fn delete_temp_file (file_path : & PathBuf) { if file_path . exists () { fs :: remove_file (file_path) . expect ("Failed to delete temp file") ; } }
}