use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for LittleEndian { fn default () -> LittleEndian { panic ! ("LittleEndian default") } }
}