// Generated macro for thir_flat (function)
macro_rules! Depcrate_thir_printthir_flat {
() => {
// Module: crate::thir::print
// Provides: {"thir_flat"}
// Dependencies: {}
# [doc = " Create a list-like THIR representation for debugging."] pub fn thir_flat (tcx : TyCtxt < '_ > , owner_def : LocalDefId) -> String { match super :: cx :: thir_body (tcx , owner_def) { Ok ((thir , _)) => format ! ("{:#?}" , thir . steal ()) , Err (_) => "error" . into () , } }
};
}
