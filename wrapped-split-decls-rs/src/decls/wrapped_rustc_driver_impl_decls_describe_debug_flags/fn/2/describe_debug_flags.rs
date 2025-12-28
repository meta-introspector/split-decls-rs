use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: describe_debug_flags");
fn describe_debug_flags () { safe_println ! ("\nAvailable options:\n") ; print_flag_list ("-Z" , config :: Z_OPTIONS) ; }
}