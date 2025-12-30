// Generated macro for impl_66 (impl)
macro_rules! Depcrateimpl_66 {
() => {
// Module: crate
// Provides: {"impl_66"}
// Dependencies: {}
impl < 'tcx > rustc_ty_utils :: sig_types :: SpannedTypeVisitor < 'tcx > for TypePrivacyVisitor < 'tcx > { type Result = ControlFlow < () > ; fn visit (& mut self , span : Span , value : impl TypeVisitable < TyCtxt < 'tcx > >) -> Self :: Result { self . span = span ; value . visit_with (& mut self . skeleton ()) } }
};
}
