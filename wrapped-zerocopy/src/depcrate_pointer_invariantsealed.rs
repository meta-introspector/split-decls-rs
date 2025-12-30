// Generated macro for sealed (module)
macro_rules! Depcrate_pointer_invariantsealed {
() => {
// Module: crate::pointer::invariant
// Provides: {"sealed"}
// Dependencies: {}
mod sealed { use super :: * ; pub trait Sealed { } impl Sealed for Shared { } impl Sealed for Exclusive { } impl Sealed for Unaligned { } impl Sealed for Aligned { } impl Sealed for Uninit { } impl Sealed for AsInitialized { } impl Sealed for Initialized { } impl Sealed for Valid { } impl < A : Sealed , AA : Sealed , V : Sealed > Sealed for (A , AA , V) { } impl Sealed for BecauseImmutable { } impl Sealed for BecauseExclusive { } }
};
}
