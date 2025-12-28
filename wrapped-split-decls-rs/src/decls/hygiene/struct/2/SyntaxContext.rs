use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A `SyntaxContext` represents a chain of pairs `(ExpnId, Transparency)` named \"marks\"."] # [doc = ""] # [doc = " See <https://rustc-dev-guide.rust-lang.org/macro-expansion.html> for more explanation."] # [derive (Clone , Copy , PartialEq , Eq , Hash)] pub struct SyntaxContext (u32) ;