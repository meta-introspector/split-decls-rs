use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct ProcMacroLoc { pub container : CrateRootModuleId , pub id : AstId < ast :: Fn > , pub expander : CustomProcMacroExpander , pub kind : ProcMacroKind , pub edition : Edition , }
}