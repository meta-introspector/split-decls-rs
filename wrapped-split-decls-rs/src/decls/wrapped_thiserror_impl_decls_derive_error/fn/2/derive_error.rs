use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro_derive (Error , attributes (backtrace , error , from , source))] pub fn derive_error (input : TokenStream) -> TokenStream { let input = parse_macro_input ! (input as DeriveInput) ; expand :: derive (& input) . into () }