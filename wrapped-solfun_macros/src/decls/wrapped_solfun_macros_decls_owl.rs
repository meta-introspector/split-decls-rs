use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl(fn, name = "owl", vis = "pub", hash = "1ecc44fa")]
pub fn owl(input: TokenStream) -> TokenStream {
    macros::owl::owl_impl(input)
}
