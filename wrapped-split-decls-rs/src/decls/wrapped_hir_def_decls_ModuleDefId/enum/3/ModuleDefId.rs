use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " The defs which can be visible in the module."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum ModuleDefId { ModuleId (ModuleId) , FunctionId (FunctionId) , AdtId (AdtId) , EnumVariantId (EnumVariantId) , ConstId (ConstId) , StaticId (StaticId) , TraitId (TraitId) , TypeAliasId (TypeAliasId) , BuiltinType (BuiltinType) , MacroId (MacroId) , }
}