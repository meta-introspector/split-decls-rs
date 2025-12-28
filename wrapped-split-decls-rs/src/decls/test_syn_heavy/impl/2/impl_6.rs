use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'ast > Visit < 'ast > for FunctionVisitor { fn visit_expr_call (& mut self , node : & 'ast ExprCall) { self . call_count += 1 ; syn :: visit :: visit_expr_call (self , node) ; } }