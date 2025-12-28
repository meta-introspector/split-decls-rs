use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , Encodable , Decodable , Debug , Default , Walkable)] pub struct FnContract { pub requires : Option < Box < Expr > > , pub ensures : Option < Box < Expr > > , }
}