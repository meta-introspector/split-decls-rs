// Generated macro for derive_zeroize_on_drop_impl (function)
macro_rules! Depcratederive_zeroize_on_drop_impl {
() => {
// Module: crate
// Provides: {"derive_zeroize_on_drop_impl"}
// Dependencies: {}
fn derive_zeroize_on_drop_impl (input : DeriveInput) -> TokenStream { let zeroizers = generate_fields (& input , quote ! { zeroize_or_on_drop }) ; let (impl_gen , type_gen , where_) = input . generics . split_for_impl () ; let name = input . ident . clone () ; let drop_impl = quote ! { impl # impl_gen Drop for # name # type_gen # where_ { fn drop (& mut self) { use :: zeroize :: __internal :: AssertZeroize ; use :: zeroize :: __internal :: AssertZeroizeOnDrop ; # zeroizers } } } ; let zeroize_on_drop_impl = impl_zeroize_on_drop (& input) ; quote ! { # drop_impl # zeroize_on_drop_impl } }
};
}
