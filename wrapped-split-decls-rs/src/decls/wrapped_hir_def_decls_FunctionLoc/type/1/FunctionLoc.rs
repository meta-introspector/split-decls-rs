use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
type FunctionLoc = AssocItemLoc < ast :: Fn > ;
}