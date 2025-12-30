// Generated macro for __cmd (function)
macro_rules! Depcrate__cmd {
() => {
// Module: crate
// Provides: {"__cmd"}
// Dependencies: {}
# [doc (hidden)] # [proc_macro] pub fn __cmd (macro_arg : TokenStream) -> TokenStream { try_cmd (macro_arg) . unwrap_or_else (| msg | parse_ts (& format ! ("compile_error!({:?})" , msg))) }
};
}
