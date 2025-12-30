// Generated macro for build_constructor (function)
macro_rules! Depcrate_arbitrarybuild_constructor {
() => {
// Module: crate::arbitrary
// Provides: {"build_constructor"}
// Dependencies: {}
fn build_constructor (path : & Path , fields : & Fields , args : TokenStream) -> TokenStream { let args = match fields { Fields :: Named (_) => quote ! { { # args } } , Fields :: Unnamed (_) => quote ! { (# args) } , Fields :: Unit => quote ! { } , } ; quote ! (# path # args) }
};
}
