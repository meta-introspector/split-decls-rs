use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: replace_lifetime");
# [doc = " Replace all lifetimes in a type with a specified one"] fn replace_lifetime (x : & Type , lt : Lifetime) -> Type { struct ReplaceLifetime (Lifetime) ; impl Fold for ReplaceLifetime { fn fold_lifetime (& mut self , _ : Lifetime) -> Lifetime { self . 0 . clone () } } ReplaceLifetime (lt) . fold_type (x . clone ()) }
}