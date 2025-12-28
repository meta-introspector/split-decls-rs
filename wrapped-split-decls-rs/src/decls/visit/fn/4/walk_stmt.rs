use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn walk_stmt < 'a , V : Visitor < 'a > > (visitor : & mut V , statement : & 'a Stmt) -> V :: Result { let Stmt { id , kind , span : _ } = statement ; try_visit ! (visitor . visit_id (* id)) ; match kind { StmtKind :: Let (local) => try_visit ! (visitor . visit_local (local)) , StmtKind :: Item (item) => try_visit ! (visitor . visit_item (item)) , StmtKind :: Expr (expr) | StmtKind :: Semi (expr) => try_visit ! (visitor . visit_expr (expr)) , StmtKind :: Empty => { } StmtKind :: MacCall (mac) => { let MacCallStmt { mac , attrs , style : _ , tokens : _ } = & * * mac ; walk_list ! (visitor , visit_attribute , attrs) ; try_visit ! (visitor . visit_mac_call (mac)) ; } } V :: Result :: output () }
}