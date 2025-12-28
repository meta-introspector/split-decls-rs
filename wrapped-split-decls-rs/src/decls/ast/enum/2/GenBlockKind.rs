use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Used to differentiate between `async {}` blocks and `gen {}` blocks."] # [derive (Clone , Encodable , Decodable , Debug , PartialEq , Eq , Walkable)] pub enum GenBlockKind { Async , Gen , AsyncGen , }