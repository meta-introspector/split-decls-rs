use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < P , T > PartialEq for TaggedRef < '_ , P , T > where P : Aligned + ? Sized , T : Tag , { # [inline] # [allow (ambiguous_wide_pointer_comparisons)] fn eq (& self , other : & Self) -> bool { self . packed == other . packed } }
}