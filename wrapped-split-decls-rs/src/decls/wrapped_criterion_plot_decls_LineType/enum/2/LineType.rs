use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Line type"] # [allow (missing_docs)] # [derive (Clone , Copy)] pub enum LineType { Dash , Dot , DotDash , DotDotDash , # [doc = " Line made of minimally sized dots"] SmallDot , Solid , }
}