// Generated macro for impl_411 (impl)
macro_rules! Depcrate_pointer_transmuteimpl_411 {
() => {
// Module: crate::pointer::transmute
// Provides: {"impl_411"}
// Dependencies: {}
unsafe impl < Src : ? Sized , Dst : ? Sized , A : Aliasing , SV : Validity , DV : Validity , R > TransmuteFromPtr < Src , A , SV , DV , R > for Dst where Dst : TransmuteFrom < Src , SV , DV > + TryTransmuteFromPtr < Src , A , SV , DV , R > , { }
};
}
