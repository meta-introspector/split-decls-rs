// Generated macro for compile_error (function)
macro_rules! Depcratecompile_error {
() => {
// Module: crate
// Provides: {"compile_error"}
// Dependencies: {}
fn compile_error (span : Span , msg : & str) -> proc_macro :: TokenStream { quote_spanned ! { span => compile_error ! (# msg) ; } . into () }
};
}
