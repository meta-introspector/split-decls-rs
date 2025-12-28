use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn apply_remove_function (ast : & mut syn :: File , details : & RemoveFunctionDetails) -> Result < () > { let mut removed = false ; ast . items . retain (| item | { if let Item :: Fn (item_fn) = item { if item_fn . sig . ident == details . function_name { removed = true ; return false ; } } true }) ; if removed { eprintln ! ("  Removed function: {}" , details . function_name) ; } else { eprintln ! ("  Warning: Function '{}' not found for removal." , details . function_name) ; } Ok (()) }