// Generated macro for impl_351 (impl)
macro_rules! Depcrate_unstable_internal_cximpl_351 {
() => {
// Module: crate::unstable::internal_cx
// Provides: {"impl_351"}
// Dependencies: {}
impl < 'tcx , T : InternalCx < 'tcx > > ExistentialTraitRefHelpers < 'tcx > for T { fn new_from_args (& self , trait_def_id : rustc_span :: def_id :: DefId , args : ty :: GenericArgsRef < 'tcx > ,) -> ty :: ExistentialTraitRef < 'tcx > { ty :: ExistentialTraitRef :: new_from_args (self . tcx () , trait_def_id , args) } }
};
}
