use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Case Insensitive wrapper of Ascii strings."] # [derive (Clone , Copy , Debug , Default)] pub struct Ascii < S > (S) ;
}