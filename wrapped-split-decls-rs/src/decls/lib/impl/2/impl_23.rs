use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for DroplessArena { # [inline] fn default () -> DroplessArena { DroplessArena { start : Cell :: new (ptr :: null_mut ()) , end : Cell :: new (ptr :: null_mut ()) , chunks : Default :: default () , } } }
}