use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct MacroCallLoc { pub def : MacroDefId , pub krate : Crate , pub kind : MacroCallKind , pub ctxt : SyntaxContext , }
}