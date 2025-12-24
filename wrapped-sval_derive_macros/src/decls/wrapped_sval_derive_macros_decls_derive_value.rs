use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro_derive(Value, attributes(sval))]
pub fn derive_value(input: TokenStream) -> TokenStream {
    TokenStream::from(derive::derive(parse_macro_input!(input as DeriveInput)))
}
