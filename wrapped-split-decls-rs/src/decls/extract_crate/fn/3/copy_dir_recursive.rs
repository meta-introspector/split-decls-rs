use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Recursively copy directory"] fn copy_dir_recursive (src : & Path , dst : & Path) -> Result < () > { fs :: create_dir_all (dst) ? ; for entry in fs :: read_dir (src) ? { let entry = entry ? ; let src_path = entry . path () ; let dst_path = dst . join (entry . file_name ()) ; if src_path . is_dir () { copy_dir_recursive (& src_path , & dst_path) ? ; } else { fs :: copy (& src_path , & dst_path) ? ; } } Ok (()) }