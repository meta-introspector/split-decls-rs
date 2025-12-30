// Generated macro for if_pub (macro)
macro_rules! Depcrate_format_description_modifierif_pub {
() => {
// Module: crate::format_description::modifier
// Provides: {"if_pub"}
// Dependencies: {}
# [doc = " Generate the provided code if and only if `pub` is present."] macro_rules ! if_pub { (pub $ (# [$ attr : meta]) *; $ ($ x : tt) *) => { $ (# [$ attr]) * # [doc = ""] # [doc = " This function exists since [`Default::default()`] cannot be used in a `const` context."] # [doc = " It may be removed once that becomes possible. As the [`Default`] trait is in the"] # [doc = " prelude, removing this function in the future will not cause any resolution failures for"] # [doc = " the overwhelming majority of users; only users who use `#![no_implicit_prelude]` will be"] # [doc = " affected. As such it will not be considered a breaking change."] $ ($ x) * } ; ($ ($ _ : tt) *) => { } ; }
};
}
