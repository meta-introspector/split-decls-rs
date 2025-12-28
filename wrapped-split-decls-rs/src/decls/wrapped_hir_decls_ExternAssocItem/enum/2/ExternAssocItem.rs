use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Invariant: `inner.as_extern_assoc_item(db).is_some()`"] # [doc = " We do not actively enforce this invariant."] # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] pub enum ExternAssocItem { Function (Function) , Static (Static) , TypeAlias (TypeAlias) , }