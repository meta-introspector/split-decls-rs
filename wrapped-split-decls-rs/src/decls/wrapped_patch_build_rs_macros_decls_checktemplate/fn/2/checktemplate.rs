use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [proc_macro] # [decl2 (fn , name = "checktemplate" , vis = "pub" , hash = "0ddf638b")] pub fn checktemplate (input : TokenStream) -> TokenStream { template_checker :: checktemplate_impl (input) }