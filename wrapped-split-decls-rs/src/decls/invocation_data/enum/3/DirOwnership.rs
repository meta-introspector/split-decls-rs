use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Describes the ownership state of a directory."] # [doc = ""] # [doc = " Used primarily for `mod` file resolution."] # [derive (Clone , Debug , PartialEq , Eq)] pub enum DirOwnership { # [doc = " The directory is owned by the current module."] Owned { # [doc = " If `Some`, the owning module is an inline module defined at the given path."] relative : Option < PathBuf > , } , # [doc = " The directory is unowned, typically via a `block`."] UnownedViaBlock , }