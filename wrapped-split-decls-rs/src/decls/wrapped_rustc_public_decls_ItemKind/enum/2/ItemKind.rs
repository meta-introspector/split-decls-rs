use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Copy , Clone , PartialEq , Eq , Debug , Hash , Serialize)] pub enum ItemKind { Fn , Static , Const , Ctor (CtorKind) , }
}