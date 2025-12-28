use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: looks_like_rust_object_file");
# [doc = " Checks if the given filename ends with the `.rcgu.o` extension that `rustc`"] # [doc = " uses for the object files it generates."] pub fn looks_like_rust_object_file (filename : & str) -> bool { let path = Path :: new (filename) ; let ext = path . extension () . and_then (| s | s . to_str ()) ; if ext != Some (OutputType :: Object . extension ()) { return false ; } let ext2 = path . file_stem () . and_then (| s | Path :: new (s) . extension ()) . and_then (| s | s . to_str ()) ; ext2 == Some (RUST_CGU_EXT) }
}