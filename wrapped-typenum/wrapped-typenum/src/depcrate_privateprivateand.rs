// Generated macro for PrivateAnd (trait)
macro_rules! Depcrate_privatePrivateAnd {
() => {
// Module: crate::private
// Provides: {"PrivateAnd"}
// Dependencies: {}
# [doc = " Does the real anding for `UInt`s; `And` just calls this and then `Trim`."] pub trait PrivateAnd < Rhs = Self > { type Output ; fn private_and (self , rhs : Rhs) -> Self :: Output ; }
};
}
