use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , PartialEq , Encodable , Decodable , Debug , Copy , Walkable)] pub enum UnsafeSource { CompilerGenerated , UserProvided , }
}