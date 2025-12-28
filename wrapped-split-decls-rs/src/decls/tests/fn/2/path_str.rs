use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn path_str (p : & str) -> String { # [cfg (not (windows))] { return p . into () ; } # [cfg (windows)] { let mut path = p . replace ('/' , "\\") ; if let Some (rest) = path . strip_prefix ('\\') { path = ["X:\\" , rest] . concat () ; } path } }