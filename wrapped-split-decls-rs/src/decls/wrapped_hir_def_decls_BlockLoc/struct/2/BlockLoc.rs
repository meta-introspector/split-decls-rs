use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Hash , PartialEq , Eq , Clone)] pub struct BlockLoc { pub ast_id : AstId < ast :: BlockExpr > , # [doc = " The containing module."] pub module : ModuleId , }