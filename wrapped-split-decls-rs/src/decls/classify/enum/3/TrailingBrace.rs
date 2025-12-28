use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
pub enum TrailingBrace < 'a > { # [doc = " Trailing brace in a macro call, like the one in `x as *const brace! {}`."] # [doc = " We will suggest changing the macro call to a different delimiter."] MacCall (& 'a ast :: MacCall) , # [doc = " Trailing brace in any other expression, such as `a + B {}`. We will"] # [doc = " suggest wrapping the innermost expression in parentheses: `a + (B {})`."] Expr (& 'a ast :: Expr) , }
}