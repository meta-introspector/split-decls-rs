use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > Frozen < T > { pub fn freeze (val : T) -> Self { Frozen (val) } }
}