use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , PartialEq , Encodable , Decodable , Debug , Copy)] pub struct KleeneToken { pub span : Span , pub op : KleeneOp , }
}