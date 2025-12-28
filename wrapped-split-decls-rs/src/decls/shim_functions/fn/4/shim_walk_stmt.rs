use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: shim_walk_stmt");
pub fn shim_walk_stmt < V : MutVisitor + ? Sized > (visitor : & mut V , stmt : & mut ast :: Stmt) { let old_stmt = std :: mem :: replace (stmt , ast :: Stmt { id : ast :: DUMMY_NODE_ID , span : DUMMY_SP , kind : ast :: StmtKind :: Empty }) ; let new_stmts = visitor . flat_map_stmt (old_stmt) ; * stmt = new_stmts . into_iter () . next () . expect ("flat_map_stmt unexpectedly returned no statements for shim_walk_stmt. A deeper refactoring of `rustc_expand` is likely required.") ; }
}