use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A block (`{ .. }`)."] # [doc = ""] # [doc = " E.g., `{ .. }` as in `fn foo() { .. }`."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct Block { # [doc = " The statements in the block."] pub stmts : ThinVec < Stmt > , pub id : NodeId , # [doc = " Distinguishes between `unsafe { ... }` and `{ ... }`."] pub rules : BlockCheckMode , pub span : Span , pub tokens : Option < LazyAttrTokenStream > , }
}