use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , Encodable , Decodable , Debug , Copy , PartialEq , Hash , HashStable_Generic)] pub struct Spanned < T > { pub node : T , pub span : Span , }
}