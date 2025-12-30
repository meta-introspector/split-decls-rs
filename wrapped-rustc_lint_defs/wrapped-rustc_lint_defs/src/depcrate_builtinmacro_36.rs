// Generated macro for macro_36 (macro)
macro_rules! Depcrate_builtinmacro_36 {
() => {
// Module: crate::builtin
// Provides: {"macro_36"}
// Dependencies: {}
declare_lint ! { # [doc = " The `unused_attributes` lint detects attributes that were not used by"] # [doc = " the compiler."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " #![ignore]"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Unused [attributes] may indicate the attribute is placed in the wrong"] # [doc = " position. Consider removing it, or placing it in the correct position."] # [doc = " Also consider if you intended to use an _inner attribute_ (with a `!`"] # [doc = " such as `#![allow(unused)]`) which applies to the item the attribute"] # [doc = " is within, or an _outer attribute_ (without a `!` such as"] # [doc = " `#[allow(unused)]`) which applies to the item *following* the"] # [doc = " attribute."] # [doc = ""] # [doc = " [attributes]: https://doc.rust-lang.org/reference/attributes.html"] pub UNUSED_ATTRIBUTES , Warn , "detects attributes that were not used by the compiler" }
};
}
