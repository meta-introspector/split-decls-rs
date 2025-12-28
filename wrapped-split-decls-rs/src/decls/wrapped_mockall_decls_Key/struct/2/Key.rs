use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Non-generic keys to `GenericExpectation` internal storage"] # [doc (hidden)] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] pub struct Key (any :: TypeId) ;
}