use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Copy , Clone , Debug , PartialEq , Encodable , Decodable)] pub enum ModuleKind { Regular , Allocator , }
}