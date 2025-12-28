use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Copy , Clone , Encodable , Decodable , Debug , PartialEq , Eq)] pub enum FormatArgPositionKind { # [doc = " `{}` or `{:.*}`"] Implicit , # [doc = " `{1}` or `{:1$}` or `{:.1$}`"] Number , # [doc = " `{a}` or `{:a$}` or `{:.a$}`"] Named , }
}