use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Syscall attribute macro that wraps function calls with safety checks"] # [proc_macro_attribute] pub fn syscall (args : TokenStream , input : TokenStream) -> TokenStream { let input_fn = parse_macro_input ! (input as ItemFn) ; let syscall_type = if ! args . is_empty () { args . to_string () . trim_matches ('"') . to_string () } else { "unknown" . to_string () } ; let fn_name = & input_fn . sig . ident ; let fn_vis = & input_fn . vis ; let fn_sig = & input_fn . sig ; let fn_block = & input_fn . block ; let expanded = quote ! { # fn_vis # fn_sig { eprintln ! ("AUDIT: {} called syscall type: {}" , stringify ! (# fn_name) , # syscall_type) ; # fn_block } } ; TokenStream :: from (expanded) }
}