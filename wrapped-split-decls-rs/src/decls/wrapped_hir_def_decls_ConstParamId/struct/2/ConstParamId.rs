use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A TypeOrConstParamId with an invariant that it actually belongs to a const"] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct ConstParamId (TypeOrConstParamId) ;
}