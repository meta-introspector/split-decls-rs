use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone , Default)] pub struct ModuleData { # [doc = " Path to the module starting from the crate name, like `my_crate::foo::bar`."] pub mod_path : Vec < Ident > , # [doc = " Stack of paths to files loaded by out-of-line module items,"] # [doc = " used to detect and report recursive module inclusions."] pub file_path_stack : Vec < PathBuf > , # [doc = " Directory to search child module files in,"] # [doc = " often (but not necessarily) the parent of the top file path on the `file_path_stack`."] pub dir_path : PathBuf , }
}