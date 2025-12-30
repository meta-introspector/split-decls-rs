// Generated macro for macro_130 (macro)
macro_rules! Depcrate_builtinmacro_130 {
() => {
// Module: crate::builtin
// Provides: {"macro_130"}
// Dependencies: {}
declare_lint ! { # [doc = " The `unnameable_types` lint detects types for which you can get objects of that type,"] # [doc = " but cannot name the type itself."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " # #![allow(unused)]"] # [doc = " #![deny(unnameable_types)]"] # [doc = " mod m {"] # [doc = "     pub struct S;"] # [doc = " }"] # [doc = ""] # [doc = " pub fn get_unnameable() -> m::S { m::S }"] # [doc = " # fn main() {}"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " It is often expected that if you can obtain an object of type `T`, then"] # [doc = " you can name the type `T` as well; this lint attempts to enforce this rule."] # [doc = " The recommended action is to either reexport the type properly to make it nameable,"] # [doc = " or document that users are not supposed to be able to name it for one reason or another."] # [doc = ""] # [doc = " Besides types, this lint applies to traits because traits can also leak through signatures,"] # [doc = " and you may obtain objects of their `dyn Trait` or `impl Trait` types."] pub UNNAMEABLE_TYPES , Allow , "effective visibility of a type is larger than the area in which it can be named" , }
};
}
