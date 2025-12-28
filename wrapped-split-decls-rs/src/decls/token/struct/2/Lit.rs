use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A literal token."] # [derive (Clone , Copy , PartialEq , Encodable , Decodable , Debug , HashStable_Generic)] pub struct Lit { pub kind : LitKind , pub symbol : Symbol , pub suffix : Option < Symbol > , }
}