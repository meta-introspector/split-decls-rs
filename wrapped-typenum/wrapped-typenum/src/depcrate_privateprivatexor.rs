// Generated macro for PrivateXor (trait)
macro_rules! Depcrate_privatePrivateXor {
() => {
// Module: crate::private
// Provides: {"PrivateXor"}
// Dependencies: {}
# [doc = " Does the real xoring for `UInt`s; `Xor` just calls this and then `Trim`."] pub trait PrivateXor < Rhs = Self > { type Output ; fn private_xor (self , rhs : Rhs) -> Self :: Output ; }
};
}
