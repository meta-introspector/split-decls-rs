use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum VisibilityKind { Public , Restricted { path : Box < Path > , id : NodeId , shorthand : bool } , Inherited , }
}