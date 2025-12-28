use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < SmolStrBuilder > for SmolStr { fn from (value : SmolStrBuilder) -> Self { value . finish () } }
}