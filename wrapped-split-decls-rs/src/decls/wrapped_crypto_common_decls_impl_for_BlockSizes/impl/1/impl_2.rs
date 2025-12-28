use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : ArraySize + sealed :: BlockSizes > BlockSizes for T { }
}