// Generated macro for Zeroize (trait)
macro_rules! DepcrateZeroize {
() => {
// Module: crate
// Provides: {"Zeroize"}
// Dependencies: {}
# [doc = " Trait for securely erasing values from memory."] pub trait Zeroize { # [doc = " Zero out this object from memory using Rust intrinsics which ensure the"] # [doc = " zeroization operation is not \"optimized away\" by the compiler."] fn zeroize (& mut self) ; }
};
}
