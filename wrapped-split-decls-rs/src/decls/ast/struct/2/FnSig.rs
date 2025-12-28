use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Represents a function's signature in a trait declaration,"] # [doc = " trait implementation, or free function."] # [derive (Clone , Encodable , Decodable , Debug)] pub struct FnSig { pub header : FnHeader , pub decl : Box < FnDecl > , pub span : Span , }