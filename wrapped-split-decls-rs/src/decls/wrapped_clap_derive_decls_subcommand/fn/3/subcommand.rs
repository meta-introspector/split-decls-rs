use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Generates the `Subcommand` impl."] # [proc_macro_derive (Subcommand , attributes (clap , command , arg , group))] pub fn subcommand (input : TokenStream) -> TokenStream { let input : DeriveInput = parse_macro_input ! (input) ; derives :: derive_subcommand (& input) . unwrap_or_else (| err | { let dummy = dummies :: subcommand (& input . ident) ; to_compile_error (err , dummy) }) . into () }
}