// Generated macro for TryZeroize (trait)
macro_rules! DepcrateTryZeroize {
() => {
// Module: crate
// Provides: {"TryZeroize"}
// Dependencies: {}
# [doc = " Fallible trait for representing cases where zeroization may or may not be"] # [doc = " possible."] # [doc = ""] # [doc = " This is primarily useful for scenarios like reference counted data, where"] # [doc = " zeroization is only possible when the last reference is dropped."] pub trait TryZeroize { # [doc = " Try to zero out this object from memory using Rust intrinsics which"] # [doc = " ensure the zeroization operation is not \"optimized away\" by the"] # [doc = " compiler."] # [must_use] fn try_zeroize (& mut self) -> bool ; }
};
}
