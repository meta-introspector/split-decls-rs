use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl(fn, name = "meme", vis = "pub", hash = "17e0d2e0")]
pub fn meme(input: TokenStream) -> TokenStream {
    macros::meme::meme_impl(input)
}
