// Generated macro for impl_const_default (macro)
macro_rules! Depcrate_format_description_modifierimpl_const_default {
() => {
// Module: crate::format_description::modifier
// Provides: {"impl_const_default"}
// Dependencies: {}
# [doc = " Implement `Default` for the given type. This also generates an inherent implementation of a"] # [doc = " `default` method that is `const fn`, permitting the default value to be used in const contexts."] macro_rules ! impl_const_default { ($ ($ (# [$ doc : meta]) * $ (@$ pub : ident) ? $ type : ty => $ default : expr ;) *) => { $ (impl $ type { if_pub ! { $ ($ pub) ? $ (# [$ doc]) *; # [inline] pub const fn default () -> Self { $ default } } } $ (# [$ doc]) * impl Default for $ type { # [inline] fn default () -> Self { $ default } }) * } ; }
};
}
