use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'ast > Visit < 'ast > for ExpressionVisitor { fn visit_expr (& mut self , expr : & 'ast Expr) { let normalized = self . normalize_expr (expr) ; if normalized . len () > 5 && normalized . contains ('(') { self . expressions . entry (normalized) . or_insert_with (Vec :: new) . push (self . current_file . clone ()) ; } syn :: visit :: visit_expr (self , expr) ; } }