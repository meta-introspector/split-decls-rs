use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Case Insensitive wrapper of strings."] # [derive (Clone , Copy)] pub struct UniCase < S > (Encoding < S >) ;
}