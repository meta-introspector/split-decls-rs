// Generated macro for impl_zeroize_on_drop (function)
macro_rules! Depcrateimpl_zeroize_on_drop {
() => {
// Module: crate
// Provides: {"impl_zeroize_on_drop"}
// Dependencies: {}
fn impl_zeroize_on_drop (input : & DeriveInput) -> TokenStream { let name = input . ident . clone () ; let (impl_gen , type_gen , where_) = input . generics . split_for_impl () ; quote ! { # [doc (hidden)] impl # impl_gen :: zeroize :: ZeroizeOnDrop for # name # type_gen # where_ { } } }
};
}
