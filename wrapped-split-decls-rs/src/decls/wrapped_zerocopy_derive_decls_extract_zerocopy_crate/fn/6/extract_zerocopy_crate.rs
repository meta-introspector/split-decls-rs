use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: extract_zerocopy_crate");
# [doc = " Attempt to extract a crate path from the provided attributes. Defaults to `::zerocopy` if not"] # [doc = " found."] fn extract_zerocopy_crate (attrs : & [Attribute]) -> Result < Path , Error > { let mut path = parse_quote ! (:: zerocopy) ; for attr in attrs { if let Meta :: List (ref meta_list) = attr . meta { if meta_list . path . is_ident ("zerocopy") { attr . parse_nested_meta (| meta | { if meta . path . is_ident ("crate") { let expr = meta . value () . and_then (| value | value . parse ()) ; if let Ok (Expr :: Lit (ExprLit { lit : Lit :: Str (lit) , .. })) = expr { if let Ok (path_lit) = lit . parse () { path = path_lit ; return Ok (()) ; } } return Err (Error :: new (Span :: call_site () , "`crate` attribute requires a path as the value" ,)) ; } Err (Error :: new (Span :: call_site () , format ! ("unknown attribute encountered: {}" , meta . path . into_token_stream ()) ,)) }) ? ; } } } Ok (path) }
}