use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A delimited token stream."] # [doc = ""] # [doc = " A `Group` internally contains a `TokenStream` which is surrounded by"] # [doc = " `Delimiter`s."] # [derive (Clone)] pub struct Group { inner : imp :: Group , }
}