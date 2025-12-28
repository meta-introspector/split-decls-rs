use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Copy , Clone , PartialEq , Encodable , Decodable , HashStable_Generic , Walkable)] pub struct DelimSpan { pub open : Span , pub close : Span , }