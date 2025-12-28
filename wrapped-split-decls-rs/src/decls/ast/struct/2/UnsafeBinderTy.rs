use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct UnsafeBinderTy { pub generic_params : ThinVec < GenericParam > , pub inner_ty : Box < Ty > , }