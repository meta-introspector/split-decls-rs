use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type StaticLoc = AssocItemLoc < ast :: Static > ;
}