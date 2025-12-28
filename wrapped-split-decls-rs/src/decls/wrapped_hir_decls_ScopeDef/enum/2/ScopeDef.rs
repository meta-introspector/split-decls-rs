use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " For IDE only"] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] pub enum ScopeDef { ModuleDef (ModuleDef) , GenericParam (GenericParam) , ImplSelfType (Impl) , AdtSelfType (Adt) , Local (Local) , Label (Label) , Unknown , }
}