// Generated macro for impl_429 (impl)
macro_rules! Depcrate_pointer_transmuteimpl_429 {
() => {
// Module: crate::pointer::transmute
// Provides: {"impl_429"}
// Dependencies: {}
unsafe impl < T > SizeEq < T > for MaybeUninit < T > { # [inline (always)] fn cast_from_raw (t : PtrInner < '_ , T >) -> PtrInner < '_ , MaybeUninit < T > > { unsafe { cast ! (t) } } }
};
}
