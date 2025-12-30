// Generated macro for impl_218 (impl)
macro_rules! Depcrate_opaque_typesimpl_218 {
() => {
// Module: crate::opaque_types
// Provides: {"impl_218"}
// Dependencies: {}
impl < 'tcx > super :: sig_types :: SpannedTypeVisitor < 'tcx > for OpaqueTypeCollector < 'tcx > { # [instrument (skip (self) , ret , level = "trace")] fn visit (& mut self , span : Span , value : impl TypeVisitable < TyCtxt < 'tcx > >) { self . visit_spanned (span , value) ; } }
};
}
