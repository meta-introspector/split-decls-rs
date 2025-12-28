use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > Default for ThinVec < T > { fn default () -> ThinVec < T > { ThinVec :: new () } }
}