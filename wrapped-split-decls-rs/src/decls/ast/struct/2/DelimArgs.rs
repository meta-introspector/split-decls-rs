use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Delimited arguments, as used in `#[attr()/[]/{}]` or `mac!()/[]/{}`."] # [derive (Clone , Encodable , Decodable , Debug , HashStable_Generic , Walkable)] pub struct DelimArgs { pub dspan : DelimSpan , pub delim : Delimiter , pub tokens : TokenStream , }
}