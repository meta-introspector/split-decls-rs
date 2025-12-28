use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl PartialEq for ProcMacro { fn eq (& self , other : & Self) -> bool { self . name == other . name && self . kind == other . kind && self . dylib_path == other . dylib_path && self . dylib_last_modified == other . dylib_last_modified && Arc :: ptr_eq (& self . process , & other . process) } }
}