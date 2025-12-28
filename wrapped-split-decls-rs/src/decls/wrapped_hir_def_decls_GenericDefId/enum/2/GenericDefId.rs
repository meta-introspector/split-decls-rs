use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , PartialOrd , Ord , Clone , Copy , PartialEq , Eq , Hash , salsa_macros :: Supertype)] pub enum GenericDefId { AdtId (AdtId) , ConstId (ConstId) , FunctionId (FunctionId) , ImplId (ImplId) , StaticId (StaticId) , TraitId (TraitId) , TypeAliasId (TypeAliasId) , }