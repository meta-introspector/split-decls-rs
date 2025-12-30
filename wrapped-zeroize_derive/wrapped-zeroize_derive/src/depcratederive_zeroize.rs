// Generated macro for derive_zeroize (function)
macro_rules! Depcratederive_zeroize {
() => {
// Module: crate
// Provides: {"derive_zeroize"}
// Dependencies: {}
# [doc = " Derive the `Zeroize` trait."] # [doc = ""] # [doc = " Supports the following attributes:"] # [doc = ""] # [doc = " On the item level:"] # [doc = " - `#[zeroize(drop)]`: *deprecated* use `ZeroizeOnDrop` instead"] # [doc = " - `#[zeroize(bound = \"T: MyTrait\")]`: this replaces any trait bounds"] # [doc = "   inferred by zeroize-derive"] # [doc = ""] # [doc = " On the field level:"] # [doc = " - `#[zeroize(skip)]`: skips this field or variant when calling `zeroize()`"] # [proc_macro_derive (Zeroize , attributes (zeroize))] pub fn derive_zeroize (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { derive_zeroize_impl (syn :: parse_macro_input ! (input as DeriveInput)) . into () }
};
}
