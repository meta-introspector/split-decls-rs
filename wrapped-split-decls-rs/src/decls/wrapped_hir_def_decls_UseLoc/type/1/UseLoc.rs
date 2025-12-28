use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
type UseLoc = ItemLoc < ast :: Use > ;
}