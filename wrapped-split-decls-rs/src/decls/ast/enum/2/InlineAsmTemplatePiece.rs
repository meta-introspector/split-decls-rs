use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , PartialEq , Encodable , Decodable , Debug , Hash , HashStable_Generic , Walkable)] pub enum InlineAsmTemplatePiece { String (Cow < 'static , str >) , Placeholder { operand_idx : usize , modifier : Option < char > , span : Span } , }
}