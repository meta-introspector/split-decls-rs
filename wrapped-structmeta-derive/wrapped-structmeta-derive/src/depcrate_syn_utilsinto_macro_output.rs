// Generated macro for into_macro_output (function)
macro_rules! Depcrate_syn_utilsinto_macro_output {
() => {
// Module: crate::syn_utils
// Provides: {"into_macro_output"}
// Dependencies: {}
pub fn into_macro_output (input : Result < TokenStream >) -> proc_macro :: TokenStream { match input { Ok (s) => s , Err (e) => e . to_compile_error () , } . into () }
};
}
