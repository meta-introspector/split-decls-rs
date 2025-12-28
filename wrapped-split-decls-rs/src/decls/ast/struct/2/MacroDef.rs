use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Represents a macro definition."] # [derive (Clone , Encodable , Decodable , Debug , HashStable_Generic , Walkable)] pub struct MacroDef { pub body : Box < DelimArgs > , # [doc = " `true` if macro was defined with `macro_rules`."] pub macro_rules : bool , }