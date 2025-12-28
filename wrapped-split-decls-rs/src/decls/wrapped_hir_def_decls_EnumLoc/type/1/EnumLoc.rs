use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type EnumLoc = ItemLoc < ast :: Enum > ;
}