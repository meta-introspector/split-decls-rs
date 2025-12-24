use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
pub fn rustc_queries(input: TokenStream) -> TokenStream {
    query::rustc_queries(input)
}
