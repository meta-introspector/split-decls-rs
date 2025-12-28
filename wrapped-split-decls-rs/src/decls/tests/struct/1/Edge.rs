use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct Edge { from : usize , to : usize , label : & 'static str , style : Style , }
}