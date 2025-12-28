use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Command-line Argument"] # [derive (Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct ParsedArg < 's > { inner : & 's OsStr , }
}