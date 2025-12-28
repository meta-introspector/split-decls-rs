use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Invariant: `inner.as_assoc_item(db).is_some()`"] # [doc = " We do not actively enforce this invariant."] # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] pub enum AssocItem { Function (Function) , Const (Const) , TypeAlias (TypeAlias) , }