use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
type ExternCrateLoc = ItemLoc < ast :: ExternCrate > ;
}