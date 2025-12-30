// Generated macro for syscall_inline (function)
macro_rules! Depcratesyscall_inline {
() => {
// Module: crate
// Provides: {"syscall_inline"}
// Dependencies: {}
# [doc = " Declarative macro to handle inline syscall syntax"] # [proc_macro] pub fn syscall_inline (input : TokenStream) -> TokenStream { let input_str = input . to_string () ; if input_str . contains ("\"read\"") { let expr = input_str . split ("\"read\"") . nth (1) . unwrap_or ("") . trim () ; let expanded = quote ! { { eprintln ! ("SYSCALL_AUDIT: read operation") ; # expr } } ; TokenStream :: from (expanded) } else if input_str . contains ("\"write\"") { let expr = input_str . split ("\"write\"") . nth (1) . unwrap_or ("") . trim () ; let expanded = quote ! { { eprintln ! ("SYSCALL_AUDIT: write operation") ; # expr } } ; TokenStream :: from (expanded) } else if input_str . contains ("\"exec\"") { let expr = input_str . split ("\"exec\"") . nth (1) . unwrap_or ("") . trim () ; let expanded = quote ! { { eprintln ! ("SYSCALL_AUDIT: exec operation") ; # expr } } ; TokenStream :: from (expanded) } else { input } }
};
}
