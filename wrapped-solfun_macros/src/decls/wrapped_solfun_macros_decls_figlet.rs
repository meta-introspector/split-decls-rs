use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl(fn, name = "figlet", vis = "pub", hash = "557915a5")]
pub fn figlet(input: TokenStream) -> TokenStream {
    macros::figlet::figlet_impl(input)
}
