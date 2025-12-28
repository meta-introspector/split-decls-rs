use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type UnionLoc = ItemLoc < ast :: Union > ;
}