use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Common trait used to represent file system paths by many Nix functions."] pub trait NixPath { # [doc = " Is the path empty?"] fn is_empty (& self) -> bool ; # [doc = " Length of the path in bytes"] fn len (& self) -> usize ; # [doc = " Execute a function with this path as a `CStr`."] # [doc = ""] # [doc = " Mostly used internally by Nix."] fn with_nix_path < T , F > (& self , f : F) -> Result < T > where F : FnOnce (& CStr) -> T ; }