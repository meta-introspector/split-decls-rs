// Generated macro for impl_400 (impl)
macro_rules! Depcrate_pointer_transmuteimpl_400 {
() => {
// Module: crate::pointer::transmute
// Provides: {"impl_400"}
// Dependencies: {}
unsafe impl < Src : ? Sized , Dst : ? Sized , A : Aliasing , SV : Validity , DV : Validity , R , S > MutationCompatible < Src , A , SV , DV , (BecauseRead , (R , S)) > for Dst where Src : Read < A , R > , Dst : Read < A , S > , { }
};
}
