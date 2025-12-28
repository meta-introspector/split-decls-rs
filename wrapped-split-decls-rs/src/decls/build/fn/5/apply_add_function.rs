use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: apply_add_function");
fn apply_add_function (ast : & mut syn :: File , details : & AddFunctionDetails) -> Result < () > { let new_fn : ItemFn = syn :: parse_str (& details . function_code) . with_context (| | format ! ("Invalid function code syntax: {}" , details . function_code)) ? ; let new_fn_name = new_fn . sig . ident . to_string () ; for item in & ast . items { if let Item :: Fn (existing_fn) = item { if existing_fn . sig . ident . to_string () == new_fn_name { eprintln ! ("  Warning: Function '{}' already exists. Skipping addition." , new_fn_name) ; return Ok (()) } } } ast . items . push (Item :: Fn (new_fn)) ; eprintln ! ("  Added function: {}" , new_fn_name) ; Ok (()) }
}