use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct Visibility { pub kind : VisibilityKind , pub span : Span , pub tokens : Option < LazyAttrTokenStream > , }