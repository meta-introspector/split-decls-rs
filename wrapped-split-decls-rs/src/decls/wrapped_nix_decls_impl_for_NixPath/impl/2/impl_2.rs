use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl NixPath for PathBuf { fn is_empty (& self) -> bool { NixPath :: is_empty (self . as_os_str ()) } fn len (& self) -> usize { NixPath :: len (self . as_os_str ()) } fn with_nix_path < T , F > (& self , f : F) -> Result < T > where F : FnOnce (& CStr) -> T , { self . as_os_str () . with_nix_path (f) } }