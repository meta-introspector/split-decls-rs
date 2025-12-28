use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Suppose we have `let _ = e` and the `order` of `e`."] # [doc = " Is the `order` such that `e` in `let _ = e` needs parentheses when it is on the RHS?"] # [doc = ""] # [doc = " Conversely, suppose that we have `(let _ = a) OP b` and `order` is that of `OP`."] # [doc = " Can we print this as `let _ = a OP b`?"] pub fn needs_par_as_let_scrutinee (order : ExprPrecedence) -> bool { order <= prec_let_scrutinee_needs_par () }