use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum StmtKind { # [doc = " A local (let) binding."] Let (Box < Local >) , # [doc = " An item definition."] Item (Box < Item >) , # [doc = " Expr without trailing semi-colon."] Expr (Box < Expr >) , # [doc = " Expr with a trailing semi-colon."] Semi (Box < Expr >) , # [doc = " Just a trailing semi-colon."] Empty , # [doc = " Macro."] MacCall (Box < MacCallStmt >) , }
}