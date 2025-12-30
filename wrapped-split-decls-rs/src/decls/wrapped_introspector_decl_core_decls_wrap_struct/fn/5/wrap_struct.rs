use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: wrap_struct");
pub fn wrap_struct (args : & DeclArgs , item : & mut ItemStruct) -> TokenStream { let name = item . ident . to_string () ; let hash = args . hash . clone () . unwrap_or_else (| | compute_hash (& item . to_token_stream () . to_string ())) ; let vis_str = match & item . vis { Visibility :: Public (_) => "pub" , Visibility :: Restricted (_) => "pub(restricted)" , Visibility :: Inherited => "private" , _ => "unknown_visibility" , } ; let line = 0 as u32 ; let module_path_str = module_path ! () . to_string () ; let registration_code = quote ! { const _ : () = { introspector_macro_helpers :: generate_decl_registration ! ("struct" , # name , # vis_str , # module_path_str , file ! () , # line , # hash) ; } ; } ; let output = quote ! { # item # registration_code } ; output . into () }
}