use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
type ExternBlockLoc = ItemLoc < ast :: ExternBlock > ;
}