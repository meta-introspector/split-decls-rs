use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Eq , PartialEq , Encodable , Decodable , Debug , HashStable_Generic)] pub struct Type { pub offset : isize , pub size : usize , pub kind : Kind , pub child : TypeTree , }