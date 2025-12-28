use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , Debug , Encodable , Decodable , HashStable)] pub struct NativeLib { pub kind : NativeLibKind , pub name : Symbol , pub filename : Option < Symbol > , pub cfg : Option < CfgEntry > , pub verbatim : bool , pub dll_imports : Vec < cstore :: DllImport > , }
}