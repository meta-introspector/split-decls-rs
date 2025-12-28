use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Copy , Eq , PartialEq , Encodable , Decodable , Debug , HashStable_Generic)] pub enum Kind { Anything , Integer , Pointer , Half , Float , Double , Unknown , }