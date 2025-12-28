use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Clone , Serialize , Deserialize , PartialEq , Eq , Hash)] enum CompilerLayer { AST , HIR , MIR , THIR , Parser , Resolve , TypeCk , Codegen , Lint , Macro , Other (String) , }
}