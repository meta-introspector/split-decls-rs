// Generated macro for impl_397 (impl)
macro_rules! Depcrate_pointer_transmuteimpl_397 {
() => {
// Module: crate::pointer::transmute
// Provides: {"impl_397"}
// Dependencies: {}
unsafe impl < Src , Dst , SV , DV > TryTransmuteFromPtr < Src , Shared , SV , DV , BecauseImmutable > for Dst where SV : Validity , DV : Validity , Src : Immutable + ? Sized , Dst : Immutable + SizeEq < Src > + ? Sized , { }
};
}
