use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A unique hash value associated to an expansion."] # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug , Encodable , Decodable , HashStable_Generic)] pub struct ExpnHash (Fingerprint) ;