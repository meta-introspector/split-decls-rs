use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Visitor to collect all top-level `use` statements."] # [derive (Default)] pub struct UseStatementCollector { pub uses : Vec < ItemUse > , }
}