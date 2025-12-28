use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct FnPtrTy { pub safety : Safety , pub ext : Extern , pub generic_params : ThinVec < GenericParam > , pub decl : Box < FnDecl > , # [doc = " Span of the `[unsafe] [extern] fn(...) -> ...` part, i.e. everything"] # [doc = " after the generic params (if there are any, e.g. `for<'a>`)."] pub decl_span : Span , }