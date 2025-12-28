use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " A list of attributes."] pub type AttrVec = ThinVec < Attribute > ;
}