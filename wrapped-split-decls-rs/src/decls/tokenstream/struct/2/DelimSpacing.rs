use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Copy , Clone , Debug , PartialEq , Encodable , Decodable , HashStable_Generic)] pub struct DelimSpacing { pub open : Spacing , pub close : Spacing , }