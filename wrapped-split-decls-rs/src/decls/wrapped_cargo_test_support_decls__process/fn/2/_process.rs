use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: _process");
fn _process (t : & OsStr) -> ProcessBuilder { let mut p = ProcessBuilder :: new (t) ; p . cwd (& paths :: root ()) . test_env () ; p }
}