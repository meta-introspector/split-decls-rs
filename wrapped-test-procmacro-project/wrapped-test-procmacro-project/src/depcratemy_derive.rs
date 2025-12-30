// Generated macro for my_derive (function)
macro_rules! Depcratemy_derive {
() => {
// Module: crate
// Provides: {"my_derive"}
// Dependencies: {}
# [doc = " Example of user-defined [derive mode macro][1]"] # [doc = ""] # [doc = " [1]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-mode-macros"] # [proc_macro_derive (MyDerive)] pub fn my_derive (_input : TokenStream) -> TokenStream { let tokens = quote ! { struct Hello ; } ; tokens . into () }
};
}
