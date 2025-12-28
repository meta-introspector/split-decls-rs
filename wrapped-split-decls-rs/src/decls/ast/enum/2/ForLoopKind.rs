use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Used to differentiate between `for` loops and `for await` loops."] # [derive (Clone , Copy , Encodable , Decodable , Debug , PartialEq , Eq , Walkable)] pub enum ForLoopKind { For , ForAwait , }