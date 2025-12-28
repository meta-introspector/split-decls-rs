use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum FnRetTy { # [doc = " Returns type is not specified."] # [doc = ""] # [doc = " Functions default to `()` and closures default to inference."] # [doc = " Span points to where return type would be inserted."] Default (Span) , # [doc = " Everything else."] Ty (Box < Ty >) , }
}