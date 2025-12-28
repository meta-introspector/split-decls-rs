use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Eq , PartialEq , Encodable , Decodable , Debug , HashStable_Generic)] pub struct FncTree { pub args : Vec < TypeTree > , pub ret : TypeTree , }