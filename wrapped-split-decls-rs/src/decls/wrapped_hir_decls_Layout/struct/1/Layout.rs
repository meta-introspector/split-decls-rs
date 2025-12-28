use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , Debug , Eq , PartialEq)] pub struct Layout (Arc < TyLayout > , Arc < TargetDataLayout >) ;
}