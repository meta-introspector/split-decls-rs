use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Helper function to get a list of config files."] # [doc = " If path is a file, returns a vector containing just that path."] # [doc = " If path is a directory, returns all .toml files within it, sorted by name."] fn get_config_paths (path : & Path) -> Result < Vec < PathBuf > > { let mut paths = Vec :: new () ; if path . is_file () { paths . push (path . to_path_buf ()) ; } else if path . is_dir () { let mut dir_entries : Vec < _ > = fs :: read_dir (path) ? . filter_map (| entry | { let entry = entry . ok () ? ; let entry_path = entry . path () ; if entry_path . is_file () && entry_path . extension () . map_or (false , | ext | ext == "toml") { Some (entry_path) } else { None } }) . collect () ; dir_entries . sort () ; paths . extend (dir_entries) ; } else { anyhow :: bail ! ("Path {:?} is neither a file nor a directory." , path) ; } Ok (paths) }
}