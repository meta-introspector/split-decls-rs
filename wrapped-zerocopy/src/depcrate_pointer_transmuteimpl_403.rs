// Generated macro for impl_403 (impl)
macro_rules! Depcrate_pointer_transmuteimpl_403 {
() => {
// Module: crate::pointer::transmute
// Provides: {"impl_403"}
// Dependencies: {}
unsafe impl < Src : ? Sized , Dst : ? Sized , A : Aliasing , SV : Validity , DV : Validity > MutationCompatible < Src , A , SV , DV , BecauseInvariantsEq > for Dst where Src : TransmuteFrom < Dst , DV , SV > , Dst : TransmuteFrom < Src , SV , DV > + InvariantsEq < Src > , { }
};
}
