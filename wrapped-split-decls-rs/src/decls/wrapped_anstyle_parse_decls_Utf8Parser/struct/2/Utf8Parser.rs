use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Allow parsing UTF-8"] # [cfg (feature = "utf8")] # [derive (Default , Clone , Debug , PartialEq , Eq)] pub struct Utf8Parser { utf8_parser : utf8 :: Parser , }
}