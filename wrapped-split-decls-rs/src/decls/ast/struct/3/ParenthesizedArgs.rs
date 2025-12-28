use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A path like `Foo(A, B) -> C`."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct ParenthesizedArgs { # [doc = " ```text"] # [doc = " Foo(A, B) -> C"] # [doc = " ^^^^^^^^^^^^^^"] # [doc = " ```"] pub span : Span , # [doc = " `(A, B)`"] pub inputs : ThinVec < Box < Ty > > , # [doc = " ```text"] # [doc = " Foo(A, B) -> C"] # [doc = "    ^^^^^^"] # [doc = " ```"] pub inputs_span : Span , # [doc = " `C`"] pub output : FnRetTy , }