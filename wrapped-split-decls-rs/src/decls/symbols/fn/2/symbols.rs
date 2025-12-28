use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub (super) fn symbols (input : TokenStream) -> TokenStream { let (mut output , errors) = symbols_with_errors (input) ; output . extend (errors . into_iter () . map (| e | e . to_compile_error ())) ; output }