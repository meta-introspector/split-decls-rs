use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A piece of a format template string."] # [doc = ""] # [doc = " E.g. \"hello\" or \"{name}\"."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum FormatArgsPiece { Literal (Symbol) , Placeholder (FormatPlaceholder) , }