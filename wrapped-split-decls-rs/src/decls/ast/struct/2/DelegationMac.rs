use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct DelegationMac { pub qself : Option < Box < QSelf > > , pub prefix : Path , pub suffixes : Option < ThinVec < (Ident , Option < Ident >) > > , pub body : Option < Box < Block > > , }
}