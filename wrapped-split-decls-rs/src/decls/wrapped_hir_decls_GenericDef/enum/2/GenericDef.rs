use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , Copy , PartialEq , Eq , Debug , Hash)] pub enum GenericDef { Function (Function) , Adt (Adt) , Trait (Trait) , TypeAlias (TypeAlias) , Impl (Impl) , Const (Const) , Static (Static) , }
}