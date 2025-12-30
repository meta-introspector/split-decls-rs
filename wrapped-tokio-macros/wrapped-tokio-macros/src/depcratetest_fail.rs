// Generated macro for test_fail (function)
macro_rules! Depcratetest_fail {
() => {
// Module: crate
// Provides: {"test_fail"}
// Dependencies: {}
# [doc = " Always fails with the error message below."] # [doc = " ```text"] # [doc = " The #[tokio::test] macro requires rt or rt-multi-thread."] # [doc = " ```"] # [proc_macro_attribute] pub fn test_fail (_args : TokenStream , _item : TokenStream) -> TokenStream { syn :: Error :: new (proc_macro2 :: Span :: call_site () , "The #[tokio::test] macro requires rt or rt-multi-thread." ,) . to_compile_error () . into () }
};
}
