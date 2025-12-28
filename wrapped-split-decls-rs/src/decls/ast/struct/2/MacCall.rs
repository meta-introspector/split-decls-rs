use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Represents a macro invocation. The `path` indicates which macro"] # [doc = " is being invoked, and the `args` are arguments passed to it."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct MacCall { pub path : Path , pub args : Box < DelimArgs > , }
}