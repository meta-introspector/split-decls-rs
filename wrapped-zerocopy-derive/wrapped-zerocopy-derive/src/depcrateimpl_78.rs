// Generated macro for impl_78 (impl)
macro_rules! Depcrateimpl_78 {
() => {
// Module: crate
// Provides: {"impl_78"}
// Dependencies: {}
impl IntoTokenStream for Result < TokenStream , Error > { fn into_ts (self) -> TokenStream { match self { Ok (ts) => ts , Err (err) => err . to_compile_error () , } } }
};
}
