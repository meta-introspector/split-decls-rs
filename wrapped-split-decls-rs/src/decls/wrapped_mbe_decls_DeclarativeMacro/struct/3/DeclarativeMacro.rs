use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " This struct contains AST for a single `macro_rules` definition. What might"] # [doc = " be very confusing is that AST has almost exactly the same shape as"] # [doc = " `tt::TokenTree`, but there's a crucial difference: in macro rules, `$ident`"] # [doc = " and `$()*` have special meaning (see `Var` and `Repeat` data structures)"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct DeclarativeMacro { rules : Box < [Rule] > , err : Option < Box < ParseError > > , }