use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
type ImplLoc = ItemLoc < ast :: Impl > ;
}