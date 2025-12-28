use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Copy , Clone , Debug , PartialEq , Encodable , Decodable)] pub enum ModuleKind { Regular , Allocator , }