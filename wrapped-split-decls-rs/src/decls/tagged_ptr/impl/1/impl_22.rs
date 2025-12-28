use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
unsafe impl < P , T > Send for TaggedRef < '_ , P , T > where P : Sync + Aligned + ? Sized , T : Send + Tag , { }
}