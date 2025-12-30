// Generated macro for PrivateSub (trait)
macro_rules! Depcrate_privatePrivateSub {
() => {
// Module: crate::private
// Provides: {"PrivateSub"}
// Dependencies: {}
# [doc = " Does the real subtraction for `UInt`s; `Sub` just calls this and then `Trim`."] pub trait PrivateSub < Rhs = Self > { type Output ; fn private_sub (self , rhs : Rhs) -> Self :: Output ; }
};
}
