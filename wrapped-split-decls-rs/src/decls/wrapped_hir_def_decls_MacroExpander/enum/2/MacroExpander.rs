use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum MacroExpander { Declarative , BuiltIn (BuiltinFnLikeExpander) , BuiltInAttr (BuiltinAttrExpander) , BuiltInDerive (BuiltinDeriveExpander) , BuiltInEager (EagerExpander) , }