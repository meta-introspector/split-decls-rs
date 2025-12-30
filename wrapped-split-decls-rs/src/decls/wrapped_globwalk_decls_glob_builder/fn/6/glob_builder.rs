use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: glob_builder");
# [doc = " Construct a new `GlobWalkerBuilder` with a glob pattern."] # [doc = ""] # [doc = " When iterated, the current directory will be recursively searched for paths"] # [doc = " matching `pattern`, unless the pattern specifies an absolute path."] pub fn glob_builder < S : AsRef < str > > (pattern : S) -> GlobWalkerBuilder { let path_pattern : PathBuf = pattern . as_ref () . into () ; if path_pattern . is_absolute () { let mut base = PathBuf :: new () ; let mut pattern = PathBuf :: new () ; let mut globbing = false ; for c in path_pattern . components () { let os = c . as_os_str () . to_str () . unwrap () ; for c in & ["*" , "{" , "}"] [..] { if os . contains (c) { globbing = true ; break ; } } if globbing { pattern . push (c) ; } else { base . push (c) ; } } let pat = pattern . to_str () . unwrap () ; if cfg ! (windows) { GlobWalkerBuilder :: new (base . to_str () . unwrap () , pat . replace ('\\' , "/")) } else { GlobWalkerBuilder :: new (base . to_str () . unwrap () , pat) } } else { GlobWalkerBuilder :: new ("." , pattern) } }
}