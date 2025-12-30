// Generated macro for impl_430 (impl)
macro_rules! Depcrate_pointer_transmuteimpl_430 {
() => {
// Module: crate::pointer::transmute
// Provides: {"impl_430"}
// Dependencies: {}
unsafe impl < T > SizeEq < MaybeUninit < T > > for T { # [inline (always)] fn cast_from_raw (t : PtrInner < '_ , MaybeUninit < T > >) -> PtrInner < '_ , T > { unsafe { cast ! (t) } } }
};
}
