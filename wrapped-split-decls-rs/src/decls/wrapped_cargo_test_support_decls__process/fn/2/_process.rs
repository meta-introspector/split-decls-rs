use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn _process (t : & OsStr) -> ProcessBuilder { let mut p = ProcessBuilder :: new (t) ; p . cwd (& paths :: root ()) . test_env () ; p }
}