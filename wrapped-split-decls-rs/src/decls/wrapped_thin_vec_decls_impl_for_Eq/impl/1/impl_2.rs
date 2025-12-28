use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > Eq for ThinVec < T > where T : Eq { }
}