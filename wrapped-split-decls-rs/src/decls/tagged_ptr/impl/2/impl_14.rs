use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < P , T > Clone for TaggedRef < '_ , P , T > where P : Aligned + ? Sized , T : Tag , { # [inline] fn clone (& self) -> Self { * self } }
}