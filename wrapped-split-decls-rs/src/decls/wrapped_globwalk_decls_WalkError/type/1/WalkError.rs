use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Error from iterating on files."] pub type WalkError = walkdir :: Error ;