use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl IncrementCounter { # [inline] fn yes (& self) -> bool { match self { IncrementCounter :: Yes => true , IncrementCounter :: No => false , } } }
}