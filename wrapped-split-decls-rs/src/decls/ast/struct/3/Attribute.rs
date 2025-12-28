use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A syntax-level representation of an attribute."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct Attribute { pub kind : AttrKind , pub id : AttrId , # [doc = " Denotes if the attribute decorates the following construct (outer)"] # [doc = " or the construct this attribute is contained within (inner)."] pub style : AttrStyle , pub span : Span , }
}