// Generated macro for error (function)
macro_rules! Depcrate_constserror {
() => {
// Module: crate::consts
// Provides: {"error"}
// Dependencies: {}
fn error (tcx : TyCtxt < '_ > , sub : GenericConstantTooComplexSub , root_span : Span ,) -> Result < ! , ErrorGuaranteed > { let reported = tcx . dcx () . emit_err (GenericConstantTooComplex { span : root_span , maybe_supported : false , sub , }) ; Err (reported) }
};
}
