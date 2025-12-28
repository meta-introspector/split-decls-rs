use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > DerefMut for ThinVec < T > { fn deref_mut (& mut self) -> & mut [T] { self . as_mut_slice () } }
}