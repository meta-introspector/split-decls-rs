use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Replace all lifetimes in a type with a specified one, AND replace all types that have a corresponding C type"] # [doc = " with the C type"] fn replace_lifetime_and_type (x : & Type , lt : Lifetime , generics_env : & HashMap < Ident , Option < Ident > > ,) -> Type { struct ReplaceLifetimeAndTy < 'a > (Lifetime , & 'a HashMap < Ident , Option < Ident > >) ; impl Fold for ReplaceLifetimeAndTy < '_ > { fn fold_lifetime (& mut self , _ : Lifetime) -> Lifetime { self . 0 . clone () } fn fold_type_path (& mut self , i : TypePath) -> TypePath { if i . qself . is_none () { if let Some (ident) = i . path . get_ident () { if let Some (Some (replacement)) = self . 1 . get (ident) { return parse_quote ! (# replacement) ; } } } fold :: fold_type_path (self , i) } } ReplaceLifetimeAndTy (lt , generics_env) . fold_type (x . clone ()) }
}