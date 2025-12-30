// Generated macro for impl_350 (impl)
macro_rules! Depcrate_unstable_internal_cximpl_350 {
() => {
// Module: crate::unstable::internal_cx
// Provides: {"impl_350"}
// Dependencies: {}
impl < 'tcx , T : InternalCx < 'tcx > > ExistentialProjectionHelpers < 'tcx > for T { fn new_from_args (& self , def_id : rustc_span :: def_id :: DefId , args : ty :: GenericArgsRef < 'tcx > , term : ty :: Term < 'tcx > ,) -> ty :: ExistentialProjection < 'tcx > { ty :: ExistentialProjection :: new_from_args (self . tcx () , def_id , args , term) } }
};
}
