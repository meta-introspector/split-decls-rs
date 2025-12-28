use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn load_declarations_recursive (dir : & str , decls : & mut Vec < Declaration >) -> Result < () > { for entry in fs :: read_dir (dir) ? { let entry = entry ? ; let path = entry . path () ; if path . is_dir () { load_declarations_recursive (& path . to_string_lossy () , decls) ? ; } else if path . extension () . map_or (false , | ext | ext == "rs") { let content = fs :: read_to_string (& path) ? ; let name = path . file_name () . unwrap () . to_string_lossy () . to_string () ; decls . push (Declaration { name , content }) ; } } Ok (()) }
}