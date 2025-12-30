// Generated macro for Reference (trait)
macro_rules! Depcrate_pointer_invariantReference {
() => {
// Module: crate::pointer::invariant
// Provides: {"Reference"}
// Dependencies: {}
# [doc = " An [`Aliasing`] invariant which is either [`Shared`] or [`Exclusive`]."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Given `A: Reference`, callers may assume that either `A = Shared` or `A ="] # [doc = " Exclusive`."] pub trait Reference : Aliasing + Sealed { }
};
}
