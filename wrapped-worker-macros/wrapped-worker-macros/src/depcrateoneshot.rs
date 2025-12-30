// Generated macro for oneshot (function)
macro_rules! Depcrateoneshot {
() => {
// Module: crate
// Provides: {"oneshot"}
// Dependencies: {}
# [proc_macro_attribute] pub fn oneshot (attr : TokenStream , item : TokenStream) -> TokenStream { let item = parse_macro_input ! (item as WorkerFn < OneshotFn >) ; let attr = parse_macro_input ! (attr as WorkerName) ; oneshot_impl (attr , item) . unwrap_or_else (| err | err . to_compile_error ()) . into () }
};
}
