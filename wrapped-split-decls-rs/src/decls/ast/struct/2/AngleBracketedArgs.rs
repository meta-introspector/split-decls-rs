use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A path like `Foo<'a, T>`."] # [derive (Clone , Encodable , Decodable , Debug , Default , Walkable)] pub struct AngleBracketedArgs { # [doc = " The overall span."] pub span : Span , # [doc = " The comma separated parts in the `<...>`."] pub args : ThinVec < AngleBracketedArg > , }
}