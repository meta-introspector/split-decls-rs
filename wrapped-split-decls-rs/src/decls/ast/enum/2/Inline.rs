use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Copy , PartialEq , Encodable , Decodable , Debug , Walkable)] pub enum Inline { Yes , No { had_parse_error : Result < () , ErrorGuaranteed > } , }