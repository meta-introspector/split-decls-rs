use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro_attribute]
pub fn before(args: TokenStream, input: TokenStream) -> TokenStream {
    expand::cfg("before", args, input)
}
