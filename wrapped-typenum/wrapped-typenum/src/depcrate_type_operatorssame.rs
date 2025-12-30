// Generated macro for Same (trait)
macro_rules! Depcrate_type_operatorsSame {
() => {
// Module: crate::type_operators
// Provides: {"Same"}
// Dependencies: {}
# [doc = " A **type operator** that ensures that `Rhs` is the same as `Self`, it is mainly useful"] # [doc = " for writing macros that can take arbitrary binary or unary operators."] # [doc = ""] # [doc = " `Same` is implemented generically for all types; it should never need to be implemented"] # [doc = " for anything else."] # [doc = ""] # [doc = " Note that Rust lazily evaluates types, so this will only fail for two different types if"] # [doc = " the `Output` is used."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " use typenum::{Same, Unsigned, U4, U5};"] # [doc = ""] # [doc = " assert_eq!(<U5 as Same<U5>>::Output::to_u32(), 5);"] # [doc = ""] # [doc = " // Only an error if we use it:"] # [doc = " # #[allow(dead_code)]"] # [doc = " type Undefined = <U5 as Same<U4>>::Output;"] # [doc = " // Compiler error:"] # [doc = " // Undefined::to_u32();"] # [doc = " ```"] pub trait Same < Rhs = Self > { # [doc = " Should always be `Self`"] type Output ; }
};
}
