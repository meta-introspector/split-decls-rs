use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl HasDataLayout for TargetDataLayout { # [inline] fn data_layout (& self) -> & TargetDataLayout { self } }
}