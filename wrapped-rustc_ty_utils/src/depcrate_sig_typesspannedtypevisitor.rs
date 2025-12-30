// Generated macro for SpannedTypeVisitor (trait)
macro_rules! Depcrate_sig_typesSpannedTypeVisitor {
() => {
// Module: crate::sig_types
// Provides: {"SpannedTypeVisitor"}
// Dependencies: {}
pub trait SpannedTypeVisitor < 'tcx > { type Result : VisitorResult = () ; fn visit (& mut self , span : Span , value : impl TypeVisitable < TyCtxt < 'tcx > >) -> Self :: Result ; }
};
}
