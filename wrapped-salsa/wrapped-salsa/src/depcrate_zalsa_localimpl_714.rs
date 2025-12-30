// Generated macro for impl_714 (impl)
macro_rules! Depcrate_zalsa_localimpl_714 {
() => {
// Module: crate::zalsa_local
// Provides: {"impl_714"}
// Dependencies: {}
impl QueryRevisionsExtraInner { # [cfg (feature = "salsa_unstable")] fn allocation_size (& self) -> usize { let QueryRevisionsExtraInner { # [cfg (feature = "accumulator")] accumulated , tracked_struct_ids , cycle_heads , iteration : _ , cycle_converged : _ , } = self ; # [cfg (feature = "accumulator")] let b = accumulated . allocation_size () ; # [cfg (not (feature = "accumulator"))] let b = 0 ; b + cycle_heads . allocation_size () + std :: mem :: size_of_val (tracked_struct_ids . as_slice ()) } }
};
}
