use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " The defs which can be visible in the module."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum ModuleDef { Module (Module) , Function (Function) , Adt (Adt) , Variant (Variant) , Const (Const) , Static (Static) , Trait (Trait) , TypeAlias (TypeAlias) , BuiltinType (BuiltinType) , Macro (Macro) , }
}