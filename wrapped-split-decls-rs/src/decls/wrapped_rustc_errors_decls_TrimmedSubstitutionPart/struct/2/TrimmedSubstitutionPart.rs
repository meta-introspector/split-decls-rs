use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , Debug , PartialEq , Hash , Encodable , Decodable)] pub struct TrimmedSubstitutionPart { pub original_span : Span , pub span : Span , pub snippet : String , }
}