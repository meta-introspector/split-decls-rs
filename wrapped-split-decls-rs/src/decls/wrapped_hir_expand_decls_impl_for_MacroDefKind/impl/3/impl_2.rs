use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl MacroDefKind { # [inline] pub fn is_declarative (& self) -> bool { matches ! (self , MacroDefKind :: Declarative (..)) } pub fn erased_ast_id (& self) -> ErasedAstId { match * self { MacroDefKind :: ProcMacro (id , ..) => id . erase () , MacroDefKind :: BuiltIn (id , _) | MacroDefKind :: BuiltInAttr (id , _) | MacroDefKind :: BuiltInDerive (id , _) | MacroDefKind :: BuiltInEager (id , _) | MacroDefKind :: Declarative (id , ..) => id . erase () , } } }