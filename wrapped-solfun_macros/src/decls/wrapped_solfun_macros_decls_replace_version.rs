use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
#[decl(fn, name = "replace_version", vis = "pub", hash = "9e3d56df")]
pub fn replace_version(input: TokenStream) -> TokenStream {
    macros::replace_version::replace_version_impl(input)
}
