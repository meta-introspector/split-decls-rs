use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct MacCallStmt { pub mac : Box < MacCall > , pub style : MacStmtStyle , pub attrs : AttrVec , pub tokens : Option < LazyAttrTokenStream > , }