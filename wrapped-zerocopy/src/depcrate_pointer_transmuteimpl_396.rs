// Generated macro for impl_396 (impl)
macro_rules! Depcrate_pointer_transmuteimpl_396 {
() => {
// Module: crate::pointer::transmute
// Provides: {"impl_396"}
// Dependencies: {}
unsafe impl < Src , Dst , SV , DV , A , R > TryTransmuteFromPtr < Src , A , SV , DV , (BecauseMutationCompatible , R) > for Dst where A : Aliasing , SV : Validity , DV : Validity , Src : TransmuteFrom < Dst , DV , SV > + ? Sized , Dst : MutationCompatible < Src , A , SV , DV , R > + SizeEq < Src > + ? Sized , { }
};
}
