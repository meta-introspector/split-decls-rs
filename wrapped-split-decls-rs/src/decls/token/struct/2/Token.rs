use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , Copy , PartialEq , Encodable , Decodable , Debug , HashStable_Generic)] pub struct Token { pub kind : TokenKind , pub span : Span , }
}