use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < P , T > Copy for TaggedRef < '_ , P , T > where P : Aligned + ? Sized , T : Tag , { }
}