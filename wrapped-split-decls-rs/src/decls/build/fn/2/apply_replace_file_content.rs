use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn apply_replace_file_content (file_path : & PathBuf , new_content : & str) -> Result < () > { fs :: write (file_path , new_content) . with_context (| | format ! ("Failed to write modified file: {:?}" , file_path)) ? ; Ok (()) }
}