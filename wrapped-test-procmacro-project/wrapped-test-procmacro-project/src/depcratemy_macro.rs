// Generated macro for my_macro (function)
macro_rules! Depcratemy_macro {
() => {
// Module: crate
// Provides: {"my_macro"}
// Dependencies: {}
# [doc = " Example of [function-like procedural macro][1]."] # [doc = ""] # [doc = " [1]: https://doc.rust-lang.org/reference/procedural-macros.html#function-like-procedural-macros"] # [proc_macro] pub fn my_macro (input : TokenStream) -> TokenStream { let input = parse_macro_input ! (input as DeriveInput) ; let tokens = quote ! { # input struct Hello ; } ; tokens . into () }
};
}
