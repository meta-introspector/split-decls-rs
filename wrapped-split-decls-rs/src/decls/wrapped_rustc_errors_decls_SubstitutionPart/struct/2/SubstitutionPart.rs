use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Debug , PartialEq , Hash , Encodable , Decodable)] pub struct SubstitutionPart { pub span : Span , pub snippet : String , }