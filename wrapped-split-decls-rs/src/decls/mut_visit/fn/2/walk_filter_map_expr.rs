use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn walk_filter_map_expr < T : MutVisitor > (vis : & mut T , mut e : Box < Expr >) -> Option < Box < Expr > > { vis . visit_expr (& mut e) ; Some (e) }
}