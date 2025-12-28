use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : ToTokens > ToTokens for QuoteOption < T > { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { tokens . append_all (match self . 0 { Some (ref t) => { quote ! { :: std :: option :: Option :: Some (# t) } } None => { quote ! { :: std :: option :: Option :: None } } }) ; } }
}