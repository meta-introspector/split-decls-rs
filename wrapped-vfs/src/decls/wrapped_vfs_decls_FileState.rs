use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum FileState {
    /// The file exists with the given content hash.
    Exists(u64),
    /// The file is deleted.
    Deleted,
    /// The file was specifically excluded by the user. We still include excluded files
    /// when they're opened (without their contents).
    Excluded,
}
