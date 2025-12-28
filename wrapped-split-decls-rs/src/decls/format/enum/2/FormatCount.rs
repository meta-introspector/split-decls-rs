use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , Encodable , Decodable , Debug , PartialEq , Eq)] pub enum FormatCount { # [doc = " `{:5}` or `{:.5}`"] Literal (u16) , # [doc = " `{:.*}`, `{:.5$}`, or `{:a$}`, etc."] Argument (FormatArgPosition) , }
}