use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < MmapMut > for MmapRaw { fn from (value : MmapMut) -> Self { Self { inner : value . inner } } }
}