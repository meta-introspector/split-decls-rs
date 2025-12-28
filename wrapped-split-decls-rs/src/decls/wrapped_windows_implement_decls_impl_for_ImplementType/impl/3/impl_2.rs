use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl ImplementType { fn to_ident (& self) -> proc_macro2 :: TokenStream { let type_name = syn :: parse_str :: < proc_macro2 :: TokenStream > (& self . type_name) . expect ("Invalid token stream") ; let generics = self . generics . iter () . map (| g | g . to_ident ()) ; quote ! { # type_name <# (# generics ,) *> } } fn to_vtbl_ident (& self) -> proc_macro2 :: TokenStream { let ident = self . to_ident () ; quote ! { <# ident as :: windows_core :: Interface >:: Vtable } } }
}