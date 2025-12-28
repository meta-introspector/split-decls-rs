use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Copy , Clone , Hash , PartialEq , Eq)] pub struct LinePosition { pub line : usize , pub column : usize , }
}