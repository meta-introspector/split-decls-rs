use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro_attribute] pub fn automock (attrs : proc_macro :: TokenStream , input : proc_macro :: TokenStream ,) -> proc_macro :: TokenStream { let attrs : proc_macro2 :: TokenStream = attrs . into () ; let input : proc_macro2 :: TokenStream = input . into () ; do_automock (attrs , input) . into () }
}