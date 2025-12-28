use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < P , T > Deref for TaggedRef < '_ , P , T > where P : Aligned + ? Sized , T : Tag , { type Target = P ; # [inline] fn deref (& self) -> & Self :: Target { self . pointer () } }
}