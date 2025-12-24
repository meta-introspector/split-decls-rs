use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro_attribute]
pub fn all(args: TokenStream, input: TokenStream) -> TokenStream {
    expand::cfg("all", args, input)
}
