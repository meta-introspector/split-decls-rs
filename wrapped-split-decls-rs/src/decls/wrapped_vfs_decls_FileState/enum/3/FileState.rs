use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Copy , Clone , Debug , PartialEq , PartialOrd)] pub enum FileState { # [doc = " The file exists with the given content hash."] Exists (u64) , # [doc = " The file is deleted."] Deleted , # [doc = " The file was specifically excluded by the user. We still include excluded files"] # [doc = " when they're opened (without their contents)."] Excluded , }