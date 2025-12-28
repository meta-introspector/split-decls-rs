use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct Symbol { name : Ident , value : Value , }
}