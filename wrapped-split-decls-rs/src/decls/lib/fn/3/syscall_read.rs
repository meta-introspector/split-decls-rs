use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Expression-level macro for audited filesystem reads"] # [proc_macro] pub fn syscall_read (input : TokenStream) -> TokenStream { let input_tokens : TokenStream2 = input . into () ; let expanded = quote ! { { eprintln ! ("SYSCALL_AUDIT: read operation") ; std :: fs :: read_dir (# input_tokens) } } ; TokenStream :: from (expanded) }