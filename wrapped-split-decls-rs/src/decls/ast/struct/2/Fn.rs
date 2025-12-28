use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Encodable , Decodable , Debug)] pub struct Fn { pub defaultness : Defaultness , pub ident : Ident , pub generics : Generics , pub sig : FnSig , pub contract : Option < Box < FnContract > > , pub define_opaque : Option < ThinVec < (NodeId , Path) > > , pub body : Option < Box < Block > > , }