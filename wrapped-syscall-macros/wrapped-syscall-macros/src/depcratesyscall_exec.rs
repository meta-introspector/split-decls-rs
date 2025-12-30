// Generated macro for syscall_exec (function)
macro_rules! Depcratesyscall_exec {
() => {
// Module: crate
// Provides: {"syscall_exec"}
// Dependencies: {}
# [doc = " Expression-level macro for audited process execution"] # [proc_macro] pub fn syscall_exec (input : TokenStream) -> TokenStream { let input_tokens : TokenStream2 = input . into () ; let expanded = quote ! { { eprintln ! ("SYSCALL_AUDIT: exec operation") ; std :: process :: Command :: new (# input_tokens) } } ; TokenStream :: from (expanded) }
};
}
