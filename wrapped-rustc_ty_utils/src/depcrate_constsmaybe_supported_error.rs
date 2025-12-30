// Generated macro for maybe_supported_error (function)
macro_rules! Depcrate_constsmaybe_supported_error {
() => {
// Module: crate::consts
// Provides: {"maybe_supported_error"}
// Dependencies: {}
fn maybe_supported_error (tcx : TyCtxt < '_ > , sub : GenericConstantTooComplexSub , root_span : Span ,) -> Result < ! , ErrorGuaranteed > { let reported = tcx . dcx () . emit_err (GenericConstantTooComplex { span : root_span , maybe_supported : true , sub , }) ; Err (reported) }
};
}
