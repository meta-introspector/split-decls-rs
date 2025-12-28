use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash)] pub struct InlineAsmOperand { owner : DefWithBodyId , expr : ExprId , index : usize , }
}