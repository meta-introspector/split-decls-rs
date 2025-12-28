use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [proc_macro] # [decl2 (fn , name = "custom_rust_driver" , vis = "pub" , hash = "a135228b")] pub fn custom_rust_driver (input : TokenStream) -> TokenStream { rustc_tracer :: custom_rust_driver_impl (input) }
}