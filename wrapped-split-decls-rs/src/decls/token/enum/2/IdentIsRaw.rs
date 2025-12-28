use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (PartialEq , Encodable , Decodable , Debug , Copy , Clone , HashStable_Generic)] pub enum IdentIsRaw { No , Yes , }
}