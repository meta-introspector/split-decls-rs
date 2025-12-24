use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl(fn, name = "daml", vis = "pub", hash = "e8455144")]
pub fn daml(input: TokenStream) -> TokenStream {
    macros::daml::daml_impl(input)
}
