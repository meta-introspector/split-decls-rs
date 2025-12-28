use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
type StructLoc = ItemLoc < ast :: Struct > ;
}