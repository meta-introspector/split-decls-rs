use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Expression-level macro for audited filesystem writes"] # [proc_macro] pub fn syscall_write (input : TokenStream) -> TokenStream { let input_tokens : TokenStream2 = input . into () ; let expanded = quote ! { { eprintln ! ("SYSCALL_AUDIT: write operation") ; std :: fs :: write (# input_tokens) } } ; TokenStream :: from (expanded) }