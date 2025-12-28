use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Like `join_path_syms`, but for `Ident`s. This function is necessary because"] # [doc = " `Ident::to_string` does more than just print the symbol in the `name` field."] pub fn join_path_idents (path : impl IntoIterator < Item = impl Borrow < Ident > >) -> String { let mut iter = path . into_iter () ; let len_hint = iter . size_hint () . 1 . unwrap_or (1) ; let mut s = String :: with_capacity (len_hint * 8) ; let first_ident = * iter . next () . unwrap () . borrow () ; if first_ident . name != kw :: PathRoot { s . push_str (& first_ident . to_string ()) ; } for ident in iter { let ident = * ident . borrow () ; debug_assert_ne ! (ident . name , kw :: PathRoot) ; s . push_str ("::") ; s . push_str (& ident . to_string ()) ; } s }
}