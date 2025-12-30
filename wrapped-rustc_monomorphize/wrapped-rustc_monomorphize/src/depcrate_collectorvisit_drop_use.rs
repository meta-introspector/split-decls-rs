// Generated macro for visit_drop_use (function)
macro_rules! Depcrate_collectorvisit_drop_use {
() => {
// Module: crate::collector
// Provides: {"visit_drop_use"}
// Dependencies: {}
fn visit_drop_use < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > , is_direct_call : bool , source : Span , output : & mut MonoItems < 'tcx > ,) { let instance = Instance :: resolve_drop_in_place (tcx , ty) ; visit_instance_use (tcx , instance , is_direct_call , source , output) ; }
};
}
