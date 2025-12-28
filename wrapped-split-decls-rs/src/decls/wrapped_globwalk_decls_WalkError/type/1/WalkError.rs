use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " Error from iterating on files."] pub type WalkError = walkdir :: Error ;
}