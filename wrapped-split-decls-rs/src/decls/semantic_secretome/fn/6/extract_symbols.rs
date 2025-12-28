use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: extract_symbols");
fn extract_symbols (file : & syn :: File , secretome : & mut SemanticSecretome , crate_name : & str) { for item in & file . items { match item { syn :: Item :: Fn (func) => { let key = format ! ("{}::{}" , crate_name , func . sig . ident) ; secretome . functions . insert (key , String :: new ()) ; secretome . total_symbols += 1 ; } syn :: Item :: Struct (s) => { let key = format ! ("{}::{}" , crate_name , s . ident) ; secretome . types . insert (key , String :: new ()) ; secretome . total_symbols += 1 ; } syn :: Item :: Enum (e) => { let key = format ! ("{}::{}" , crate_name , e . ident) ; secretome . types . insert (key , String :: new ()) ; secretome . total_symbols += 1 ; } syn :: Item :: Const (c) => { let key = format ! ("{}::{}" , crate_name , c . ident) ; secretome . constants . insert (key , String :: new ()) ; secretome . total_symbols += 1 ; } _ => { } } } }
}