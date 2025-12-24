use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro_attribute]
pub fn since(args: TokenStream, input: TokenStream) -> TokenStream {
    expand::cfg("since", args, input)
}
