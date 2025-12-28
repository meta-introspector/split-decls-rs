use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl ExpandedStmtHasAttrs for ExpandedStmt { fn visit_stmt_attrs (& mut self , f : impl FnOnce (& mut AttrVec)) { match & mut self . 0 . kind { ast :: StmtKind :: Let (local) => f (& mut local . attrs) , ast :: StmtKind :: Item (item) => f (& mut item . attrs) , ast :: StmtKind :: Expr (expr) | ast :: StmtKind :: Semi (expr) => f (& mut expr . attrs) , ast :: StmtKind :: MacCall (mac) => f (& mut mac . attrs) , ast :: StmtKind :: Empty => { } , } } }
}