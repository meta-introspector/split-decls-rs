use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " 8-dimensional point in the Bott tower"] # [derive (Debug , Clone , serde :: Serialize , serde :: Deserialize)] pub struct Level8DPoint { pub coordinates : [f64 ; 8] , pub level : usize , pub generation : usize , pub cached_result : Option < String > , }
}