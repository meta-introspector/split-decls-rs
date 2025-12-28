use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Was parsing recovery performed?"] # [derive (Copy , Clone , Debug , Encodable , Decodable , HashStable_Generic , Walkable)] pub enum Recovered { No , Yes (ErrorGuaranteed) , }