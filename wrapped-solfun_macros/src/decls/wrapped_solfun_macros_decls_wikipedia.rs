use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl(fn, name = "wikipedia", vis = "pub", hash = "616a7bb3")]
pub fn wikipedia(input: TokenStream) -> TokenStream {
    macros::wikipedia::wikipedia_impl(input)
}
