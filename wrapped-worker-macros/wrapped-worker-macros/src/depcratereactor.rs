// Generated macro for reactor (function)
macro_rules! Depcratereactor {
() => {
// Module: crate
// Provides: {"reactor"}
// Dependencies: {}
# [proc_macro_attribute] pub fn reactor (attr : TokenStream , item : TokenStream) -> TokenStream { let item = parse_macro_input ! (item as WorkerFn < ReactorFn >) ; let attr = parse_macro_input ! (attr as WorkerName) ; reactor_impl (attr , item) . unwrap_or_else (| err | err . to_compile_error ()) . into () }
};
}
