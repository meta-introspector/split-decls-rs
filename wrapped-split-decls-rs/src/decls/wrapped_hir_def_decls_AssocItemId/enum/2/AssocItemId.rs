use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Copy , Clone , PartialEq , Eq , Hash , salsa_macros :: Supertype)] pub enum AssocItemId { FunctionId (FunctionId) , ConstId (ConstId) , TypeAliasId (TypeAliasId) , }
}