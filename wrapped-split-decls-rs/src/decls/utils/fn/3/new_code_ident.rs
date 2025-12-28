use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: new_code_ident");
# [doc = " Returns an ident of the form `__code_N` where `N` is incremented once with every call."] pub (crate) fn new_code_ident () -> syn :: Ident { CODE_IDENT_COUNT . with (| count | { let ident = format_ident ! ("__code_{}" , * count . borrow ()) ; * count . borrow_mut () += 1 ; ident }) }
}