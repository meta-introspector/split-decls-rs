// Generated macro for impl_352 (impl)
macro_rules! Depcrate_unstable_internal_cximpl_352 {
() => {
// Module: crate::unstable::internal_cx
// Provides: {"impl_352"}
// Dependencies: {}
impl < 'tcx , T : InternalCx < 'tcx > > TraitRefHelpers < 'tcx > for T { fn new_from_args (& self , trait_def_id : rustc_span :: def_id :: DefId , args : ty :: GenericArgsRef < 'tcx > ,) -> ty :: TraitRef < 'tcx > { ty :: TraitRef :: new_from_args (self . tcx () , trait_def_id , args) } }
};
}
