use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn apply_replace_file_content_from_file (target_file : & PathBuf , source_file : & PathBuf) -> Result < () > { let new_content = fs :: read_to_string (source_file) . with_context (| | format ! ("Failed to read source file: {:?}" , source_file)) ? ; fs :: write (target_file , new_content) . with_context (| | format ! ("Failed to write modified file: {:?}" , target_file)) ? ; Ok (()) }
}