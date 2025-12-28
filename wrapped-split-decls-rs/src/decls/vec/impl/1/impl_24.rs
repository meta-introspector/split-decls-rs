use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
unsafe impl < I : Idx , T > Send for IndexVec < I , T > where T : Send { }
}