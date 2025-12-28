use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl ModuleData { pub fn with_dir_path (& self , dir_path : PathBuf) -> ModuleData { ModuleData { mod_path : self . mod_path . clone () , file_path_stack : self . file_path_stack . clone () , dir_path , } } }
}