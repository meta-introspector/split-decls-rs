use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Expression-level macro for audited filesystem file reads"] # [proc_macro] pub fn syscall_read_file (input : TokenStream) -> TokenStream { let input_tokens : TokenStream2 = input . into () ; let expanded = quote ! { { eprintln ! ("SYSCALL_AUDIT: read_file operation") ; std :: fs :: read (# input_tokens) } } ; TokenStream :: from (expanded) }