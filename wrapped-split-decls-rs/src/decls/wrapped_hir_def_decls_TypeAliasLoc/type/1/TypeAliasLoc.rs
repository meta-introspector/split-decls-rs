use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
type TypeAliasLoc = AssocItemLoc < ast :: TypeAlias > ;
}