use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: build_uuid");
fn build_uuid (input : TokenStream) -> Result < TokenStream , Error > { let str_lit = match syn :: parse :: < syn :: Lit > (input) { Ok (syn :: Lit :: Str (literal)) => literal , _ => return Err (Error :: NonStringLiteral) , } ; let bytes = parser :: try_parse (& str_lit . value ()) . map_err (| e | Error :: UuidParse (str_lit , e . into_err ())) ? ; let tokens = bytes . iter () . map (| byte | { quote ! { # byte , } }) . collect :: < TokenStream2 > () ; Ok (quote ! { [# tokens] } . into ()) }
}