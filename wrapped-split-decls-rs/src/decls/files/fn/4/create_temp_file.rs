use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: create_temp_file");
# [allow (dead_code)] pub fn create_temp_file (content : & str , extension : & str) -> PathBuf { let mut path = PathBuf :: from ("tests/temp") ; fs :: create_dir_all (& path) . expect ("Failed to create temp directory") ; let random_name = Uuid :: new_v4 () . to_string () ; path . push (format ! ("{}.{}" , random_name , extension)) ; let mut file = File :: create (& path) . expect ("Failed to create temp file") ; file . write_all (content . as_bytes ()) . expect ("Failed to write to temp file") ; path }
}