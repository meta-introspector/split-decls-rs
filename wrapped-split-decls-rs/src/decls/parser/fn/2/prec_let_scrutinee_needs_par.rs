use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " In `let p = e`, operators with precedence `<=` this one requires parentheses in `e`."] pub fn prec_let_scrutinee_needs_par () -> ExprPrecedence { ExprPrecedence :: LAnd }
}