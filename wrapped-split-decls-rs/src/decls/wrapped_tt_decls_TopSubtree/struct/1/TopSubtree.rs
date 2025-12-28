use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , PartialEq , Eq , Hash)] pub struct TopSubtree < S > (pub Box < [TokenTree < S >] >) ;
}