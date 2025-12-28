use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a > VisitMut for ExpressionReplacer < 'a > { fn visit_item_fn_mut (& mut self , i : & mut ItemFn) { if i . sig . ident == self . function_name { self . visit_block_mut (& mut i . block) ; } visit_item_fn_mut (self , i) ; } fn visit_expr_mut (& mut self , i : & mut syn :: Expr) { use quote :: ToTokens ; let mut i_tokens = proc_macro2 :: TokenStream :: new () ; i . to_tokens (& mut i_tokens) ; let mut old_snippet_tokens = proc_macro2 :: TokenStream :: new () ; self . old_snippet . to_tokens (& mut old_snippet_tokens) ; if i_tokens . to_string () == old_snippet_tokens . to_string () { * i = self . new_snippet . clone () ; self . replaced_count += 1 ; } syn :: visit_mut :: visit_expr_mut (self , i) ; } }
}