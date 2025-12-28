use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn wrap_for_syn_context (trait_code : TokenStream) -> TokenStream { quote ! { use syn :: { Item , parse2 } ; use proc_macro2 :: TokenStream ; use quote :: ToTokens ; pub struct SynContext ; impl UniversalAst for SynContext { type TokenStream = TokenStream ; type Item = Item ; type Error = syn :: Error ; fn parse_item (& self , input : Self :: TokenStream) -> Result < Self :: Item , Self :: Error > { parse2 (input) } fn transform_item (& self , item : Self :: Item) -> Result < Self :: Item , Self :: Error > { Ok (item) } fn generate_code (& self , item : Self :: Item) -> Self :: TokenStream { item . to_token_stream () } } # trait_code } }
}