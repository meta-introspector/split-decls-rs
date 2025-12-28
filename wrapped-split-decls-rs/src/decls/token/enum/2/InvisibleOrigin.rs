use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Copy , Clone , PartialEq , Debug , Encodable , Decodable , HashStable_Generic)] pub enum InvisibleOrigin { MetaVar (MetaVarKind) , ProcMacro , }
}