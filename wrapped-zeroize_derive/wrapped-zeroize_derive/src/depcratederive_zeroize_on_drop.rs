// Generated macro for derive_zeroize_on_drop (function)
macro_rules! Depcratederive_zeroize_on_drop {
() => {
// Module: crate
// Provides: {"derive_zeroize_on_drop"}
// Dependencies: {}
# [doc = " Derive the `ZeroizeOnDrop` trait."] # [doc = ""] # [doc = " Supports the following attributes:"] # [doc = ""] # [doc = " On the field level:"] # [doc = " - `#[zeroize(skip)]`: skips this field or variant when calling `zeroize()`"] # [proc_macro_derive (ZeroizeOnDrop , attributes (zeroize))] pub fn derive_zeroize_on_drop (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { derive_zeroize_on_drop_impl (syn :: parse_macro_input ! (input as DeriveInput)) . into () }
};
}
