use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A statement. No `attrs` or `tokens` fields because each `StmtKind` variant"] # [doc = " contains an AST node with those fields. (Except for `StmtKind::Empty`,"] # [doc = " which never has attrs or tokens)"] # [derive (Clone , Encodable , Decodable , Debug)] pub struct Stmt { pub id : NodeId , pub kind : StmtKind , pub span : Span , }