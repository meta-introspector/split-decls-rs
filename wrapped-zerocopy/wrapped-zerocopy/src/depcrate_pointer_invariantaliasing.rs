// Generated macro for Aliasing (trait)
macro_rules! Depcrate_pointer_invariantAliasing {
() => {
// Module: crate::pointer::invariant
// Provides: {"Aliasing"}
// Dependencies: {}
# [doc = " The aliasing invariant of a [`Ptr`][super::Ptr]."] # [doc = ""] # [doc = " All aliasing invariants must permit reading from the bytes of a pointer's"] # [doc = " referent which are not covered by [`UnsafeCell`]s."] # [doc = ""] # [doc = " [`UnsafeCell`]: core::cell::UnsafeCell"] pub trait Aliasing : Sealed { # [doc = " Is `Self` [`Exclusive`]?"] # [doc (hidden)] const IS_EXCLUSIVE : bool ; }
};
}
