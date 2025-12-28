use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl IntoU32 for u8 { fn into_u32 (self) -> u32 { self . into () } }
}