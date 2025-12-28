use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Whether we're unwrapping or wrapping an unsafe binder"] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] # [derive (Encodable , Decodable , HashStable_Generic , Walkable)] pub enum UnsafeBinderCastKind { Wrap , Unwrap , }