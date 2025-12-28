use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn walk_flat_map_stmt < T : MutVisitor > (vis : & mut T , Stmt { kind , span , mut id } : Stmt ,) -> SmallVec < [Stmt ; 1] > { vis . visit_id (& mut id) ; let mut stmts : SmallVec < [Stmt ; 1] > = walk_flat_map_stmt_kind (vis , kind) . into_iter () . map (| kind | Stmt { id , kind , span }) . collect () ; match & mut stmts [..] { [] => { } [stmt] => vis . visit_span (& mut stmt . span) , _ => panic ! ("cloning statement `NodeId`s is prohibited by default, \
             the visitor should implement custom statement visiting") , } stmts }