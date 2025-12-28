use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct MacroRulesLoc { pub container : ModuleId , pub id : AstId < ast :: MacroRules > , pub expander : MacroExpander , pub flags : MacroRulesLocFlags , pub edition : Edition , }