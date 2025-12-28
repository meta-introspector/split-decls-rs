use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: parse_ident");
# [doc = " Parses an generic ident"] fn parse_ident < 'psess > (iter : & mut TokenStreamIter < '_ > , psess : & 'psess ParseSess , fallback_span : Span ,) -> PResult < 'psess , Ident > { let token = parse_token (iter , psess , fallback_span) ? ; parse_ident_from_token (psess , token) }
}