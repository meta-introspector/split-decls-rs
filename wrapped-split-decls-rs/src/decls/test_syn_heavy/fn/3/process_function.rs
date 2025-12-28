use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn process_function (input : TokenStream) -> TokenStream { let parsed : syn :: ItemFn = syn :: parse2 (input) . unwrap () ; let fn_name = & parsed . sig . ident ; let inputs = & parsed . sig . inputs ; let output = & parsed . sig . output ; let block = & parsed . block ; let mut visitor = FunctionVisitor :: new () ; visitor . visit_item_fn (& parsed) ; quote ! { pub fn # fn_name (# inputs) # output { println ! ("Wrapped function: {}" , stringify ! (# fn_name)) ; # block } } }